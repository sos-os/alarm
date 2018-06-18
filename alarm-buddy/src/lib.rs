// ••• ALARM: the SOS memory allocator
// --- by Eliza Weisman (eliza@elizas.website)
// ••• and the SOS contributors
//
//  Copyright (c) 2018 Eliza Weisman
//  Released under the terms of the MIT license. See `LICENSE` in the root
//  directory of this repository for more information.
//
//! ALARM Buddy-Block Allocator
#![feature(alloc, allocator_api)]
#![no_std]
extern crate alloc;
extern crate alarm_base;
extern crate hal9000;
extern crate intruder_alarm;
extern crate spin;

use core::{
    cmp::min,
    default::Default,
    ops,
    ptr::NonNull,
};

use alarm_base::{AllocResult, FrameAllocator};
use alloc::alloc::{Alloc, AllocErr, Layout};
use hal9000::mem::{Page, PhysicalAddress};
use intruder_alarm::{
    list::{List, Linked, Links},
    UnsafeRef,
};


pub type FreeList = List<FreeBlock, FreeBlock, UnsafeRef<FreeBlock>>;

mod log2;

// #[cfg(test)]
// mod tests;

use self::log2::Log2;

/// A free block header.
#[derive(Debug, Default)]
pub struct FreeBlock {

    /// The size of the block (in bytes), including the free block header.
    pub size: usize,

    /// Pointers to the previous and next blocks in the free list.
    links: Links<FreeBlock>,

}

/// A buddy-block allocator.
pub struct Heap<'a, F: 'a> {

    /// The heap's minimum block size.
    pub min_block_size: usize,

    /// Log base 2 of the minimum block size.
    ///
    /// We cache this to avoid re-calculating.
    min_block_size_log2: usize,

    /// The number of blocks in the heap.
    ///
    /// This must always be a power of 2.
    heap_size: usize,

    /// The allocator's array of free list heads.
    free_lists: &'a mut [FreeList],

    /// A pointer to the base of the heap.
    base_ptr: *mut u8,

    /// The underlying frame provider.
    frames: &'a mut F,

}

impl<'a, F> Heap<'a, F>
where
    F: FrameAllocator
{

    /// Computes the size of an allocation request.
    ///
    /// # Arguments
    /// + `layout`: the `Layout` to compute the size for.
    pub fn block_size(&self, layout: &Layout) -> AllocResult<usize> {
        let align = layout.align();

        // We cannot allocate layouts whose alignments are not powers of 2.
        if !align.is_power_of_two() {
            // TODO: "Unsupported alignment (not a power of 2)."
            return Err(AllocErr);
        }

        // We cannot allocate layouts with alignments greater than the
        // heap's base alignment (the size of a frame in the underlying
        // frame provider).
        if align > F::FRAME_SIZE {
            // TODO: log "Unsupported alignment (exceeded FRAME_SIZE)."
            return Err(AllocErr)?;
        }

        // The allocation's size must be at least as large as the layout's
        // alignment.
        let size = usize::max(layout.size(), layout.align());

        // Round up to the heap's minimum block size.
        let size = usize::max(size, self.min_block_size);

        // Round up to the next power oftwo.
        let size = size.next_power_of_two();

        // If the resulting size of the allocation is greater than the
        // size of the heap,we (obviously) cannot allocate the request.
        if size > self.heap_size {
            // TODO: log "Unsupported size for alignment (exceeded heap size)."
            return Err(AllocErr);
        }

        Ok(size)
    }

    /// Compute the order of the free list for a given `Layout`.
    pub fn block_order(&self, layout: &Layout) -> AllocResult<usize> {
        self.block_size(layout).map(|size| self.order_from_size(size))
    }

    #[inline]
    fn order_from_size(&self, size: usize) -> usize {
        size.log2() - self.min_block_size_log2
    }

    /// Push a `FreeBlock` onto the corresponding free list.
    ///
    /// The order of the free list to push to is calculated based on
    /// the `FreeBlock`'s `size` field.
    ///
    /// # Arguments
    /// - `block`: a `*mut` pointer to the block to push.
    ///
    /// # Safety
    /// This function is unsafe due to:
    /// - dereference of raw `*mut` pointer
    /// - if the `size` field of the pushed block header is _less_ than
    ///   the actual size of the block, then the remaining memory will
    ///   be leaked.
    /// - if the `size` field of the pushed block is _greater_ than the
    ///   actual size of the block, then that block may be allocated for
    ///   requests which exceed the block's size. This may result in
    ///   writes to already allocated memory.
    #[inline]
    pub unsafe fn push_block(&mut self, block: NonNull<FreeBlock>) {
        let order = self.order_from_size((*block).size);
        self.push_block_order(block, order);
    }

    #[inline]
    unsafe fn push_block_order(&mut self, block: NonNull<FreeBlock>, order: usize) {
        let block_ref = UnsafeRef::from_mut_ptr(block);
        self.free_lists[order].push_front_node(block_ref);
    }

    /// Returns the `buddy` for a given block, if it exists.
    pub unsafe fn get_buddy(&self, block: NonNull<FreeBlock>, order: usize)
        -> Option<NonNull<FreeBlock>> {
        let size = 1 << (self.min_block_size_log2 as usize + order);

        // If the block is the size of the entire heap, it obviously
        // can't have a buddy.
        if size == self.heap_size {
            return None;
        }

        let relative_offset = (block as usize) - (self.base_ptr as usize);
        let buddy_offset = (relative_offset ^ size) as isize;
        Some(self.base_ptr.offset(buddy_offset) as *mut _)
    }

}

impl<'a, F> Heap<'a, F>
where
    F: FrameAllocator,
    <<F as FrameAllocator>::Frame as Page>::Address:  PhysicalAddress,
{

    /// Request a new page from the frame allocator, and push it to
    /// the heap's free list corresponding to the allocated frame size.
    ///
    /// # Returns
    /// - `Ok(())` if the allocator was successfully refilled.
    /// - `Err(AllocErr)` if an error occurred allocating a new block from
    ///   the underlying frame allocator.
    ///
    /// # Safety
    /// This function is unsafe due to use of unsafe APIs. It could
    /// potentially become safe if it is refactored o better enforce
    /// invariants across unsafe API calls.
    ///
    pub unsafe fn refill(&mut self) -> Result<(), AllocErr> {
        // Allocate a new frame from `self.frames` and return
        // `Err` if the allocation failed.
        let new_frame = self.frames.alloc()?;

        // Write the free block header into the new frame, and
        // push it to the appropriate free list.
        let new_block = FreeBlock::from_frame(new_frame);
        self.push_block(new_block);

        // Refilling the heap with a new block increases the overall
        // size of the heap.
        self.heap_size += F::FRAME_SIZE;

        Ok(())
    }

}

unsafe impl<'a, F> Alloc for Heap<'a, F>
where
    F: FrameAllocator,
    <<F as FrameAllocator>::Frame as Page>::Address:  PhysicalAddress,
{

    /// Allocate a block for the given order.
    ///
    /// This is the core of the buddy-block allocation algorithm. Here's
    /// a simple description of how it works:
    ///
    /// 1. Let _o_ be the desired order for the allocation.
    /// 2. Find a free block with order _o_. If found, return that block.
    /// 3. Otherwise, find a free block with order _x_ > _o_, minimizing _x_.
    ///    - If found:
    ///      - Split the block in two, creating two free blocks with order
    ///        _x_-1. These two blocks are called order-(_x_-1) buddies, because
    ///        they are adjacent blocks with order _x_-1, and the address of one
    ///        is easily calculated from the address of the other.
    ///      - Repeat step 2.
    ///    - If there’s no larger free block:
    ///        - The allocation fails: return OOM.
    unsafe fn alloc(&mut self, layout: Layout) -> AllocResult<NonNull<u8>> {
        // Try to calculate the minimum order necessary for the requested
        // layout, or return `AllocErr::Unsupported` if the layout is
        // invalid.
        let min_order = self.block_order(&layout)?;

        // Iterate over the free lists starting at the desired order to
        // search for a free block.
        for current_order in min_order..self.free_lists.len() {
            // Try to pop a block off the free list, returning `None` if
            // that free list is empty. If the free list is empty, continue to
            // the next free list.
            if let Some(mut block) = self
                .free_lists[current_order]
                .pop_front_node()
            {
                let block = block.as_mut_ptr();

                // If the current order is greater than the minimum required
                // order for the allocation, split the block in half until it
                // matches the requested order.
                for split_order in current_order..min_order {
                    // Split `block` in half, returning a pointer to the free
                    // block header at the beginning of the split off half.
                    // `block` is unchanged and still points to the header
                    // at the beginning of the block.
                    let split = FreeBlock::split(block);
                    // Push `split` onto the free list for `split_order`.
                    self.push_block_order(split, split_order);
                }

                return Ok(block as *mut _);
            }
        }

        // We were not able to allocate a block. Refill the heap and try again.
        // TODO: this could be optimized by making it iterative rather than
        //       recursive...
        // TODO: upper bound on number of times the allocator can be refilled?
        // TODO: nicer error?
        // let err = AllocErr::Exhausted { request: layout.clone() };
        self.refill()?;
        self.alloc(layout)
    }

    unsafe fn dealloc(&mut self, ptr: NonNull<u8>, layout: Layout) {

        let mut order = self.block_order(&layout)
            .expect("can't deallocate an invalid layout");

        let mut block = FreeBlock::from_ptr_size(ptr.cast::<FreeBlock>(), layout.size());
        // Iterate over the free lists starting at the desired order to
        // search for a free block.
        while let Some(buddy) = self.get_buddy(block, order) {
            if self.free_lists[order].cursor_mut().find_and_remove(|checking| checking as *mut _ == buddy) {
                block = FreeBlock::merge(block, buddy);
            } else {
                break;
            }
        }

        self.push_block_order(block, order);
    }

}

// ===== impl FreeBlock =====

impl Linked for FreeBlock {
    #[inline]
    fn links(&self) -> &Links<Self> {
        &self.links
    }

    #[inline]
    fn links_mut(&mut self) -> &mut Links<Self> {
        &mut self.links
    }
}


impl FreeBlock {

    /// Construct a new unlinked free block header in the given frame and
    /// return it.
    #[inline]
    unsafe fn from_frame<F, A>(frame: F) -> NonNull<Self>
    where
        F: Page<Address = A>,
        A: PhysicalAddress,
    {
        // Get a mutable reference to the frame's start address. We will
        // use this pointer to write the free block header, and then return it.
        let ptr = NonNull::new(frame.base_address().as_mut_ptr())
            // TOOD: do we trust frame start addresses enough to use
            //      `NonNull::new_unchecked` here instead? I don't know...
            .expect("frame start address was null!");
        Self::from_ptr_size(ptr, F::SIZE)
    }

    /// Returns a new, unlinked `FreeBlock` header at the given pointer
    /// with the provided size.
    ///
    /// # Safety
    /// This function is unsafe because:
    /// - use of raw `*mut` pointers
    /// - `size` MUST match the size of the free block
    #[inline]
    unsafe fn from_ptr_size(ptr: NonNull<FreeBlock>, size: usize) -> NonNull<Self> {

        // Write the free block header into the frame.
        *ptr.as_mut() = FreeBlock {
            size,
            ..Default::default()
        };

        ptr
    }

    /// Split `block` in half, returning a new pointer to the half split off.
    ///
    /// `block` still points to the beginning of the free block, while the
    /// returned pointer points to a new free block header in the half that was
    /// split.
    unsafe fn split(block: NonNull<FreeBlock>) -> NonNull<Self> {
        let block = block.as_ptr();
        //   H_______________
        //   ^
        // `block` points to the header of the block.
        (*block).size >>= 1;
        let size = (*block).size;
        let split_ptr = NonNull::new_unchecked(block.offset(size as isize));
        //   H_______________
        //   ^       ^
        //   |  `split_ptr` now points to an address halfway into the free block
        // `block` still points to the header of the block.
        FreeBlock::from_ptr_size(split_ptr, size)
        //   H_______H_______
        //   ^       ^
        // `from_ptr_size` inserts a new free block header in the address
        // pointed to by `split_ptr`
    }

    #[inline]
    unsafe fn merge(a: NonNull<FreeBlock>, b: NonNull<FreeBlock>) -> NonNull<FreeBlock> {
        // select the block with the lower address.
        let block = min(a, b);

        // sum the sizes of the merged blocks and set the size of the new
        // block equal to the sum.
        block.as_mut().size = a.as_ref().size + b.as_ref().size;

        // return the merged block pointer
        block
    }

}
