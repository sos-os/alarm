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
extern crate intruder_alarm;
extern crate spin;

use alarm_base::FrameAllocator;

use intruder_alarm::list::{List, Linked, Links};
use intruder_alarm::UnsafeRef;

use alloc::allocator::{AllocErr, Layout};

pub type FreeList = List<FreeBlock, FreeBlock, UnsafeRef<FreeBlock>>;

mod log2;

// #[cfg(test)]
// mod tests;

use self::log2::Log2;

/// A free block header.
pub struct FreeBlock {
    links: Links<FreeBlock>
}

/// A buddy-block allocator.
pub struct Heap<'a, F: 'a> {

    /// The heap's minimum block size.
    pub min_block_size: usize,

    /// Log base 2 of the minimum block size.
    ///
    /// We cache this to avoid re-calculating.
    min_block_size_log2: usize,

    /// The order of

    /// The number of blocks in the heap.
    ///
    /// This must always be a power of 2.
    heap_size: usize,

    /// The allocator's array of free list heads.
    free_lists: &'a mut [FreeList],

    /// The underlying frame provider.
    frames: &'a mut F,

}

impl<'a, F> Heap<'a, F> {

    /// Push a block of the given order.
    //
    // TODO: should this take a `Layout` instead?
    unsafe fn push_block(&mut self, ptr: *mut u8, order: usize) {
        unimplemented!()
    }

}

impl<'a, F> Heap<'a, F>
where
    F: FrameAllocator
{

    /// Computes the size of an allocation request.
    ///
    /// # Arguments
    /// + `layout`: the `Layout` to compute the size for.
    pub fn block_size(&self, layout: &Layout) -> Result<usize, AllocErr> {
        let align = layout.align();

        // We cannot allocate layouts whose alignments are not powers of 2.
        if !align.is_power_of_two() {
            Err(AllocErr::Unsupported {
                details: "Unsupported alignment (not a power of 2)."
            })?;
        }

        // We cannot allocate layouts with alignments greater than the
        // heap's base alignment (the size of a frame in the underlying
        // frame provider).
        if align > F::FRAME_SIZE {
            Err(AllocErr::Unsupported {
                details: "Unsupported alignment (exceeded FRAME_SIZE)."
            })?;
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
            Err(AllocErr::Unsupported {
                details: "Unsupported size for alignment (exceeded heap size)."
            })?;
        }

        Ok(size)
    }

    /// Compute the order of the free list for a given `Layout`.
    pub fn block_order(&self, layout: &Layout) -> Result<usize, AllocErr> {
        let size = self.block_size(layout)?;
        Ok(size.log2() - self.min_block_size_log2)
    }

    unsafe fn refill(&mut self) -> Result<(), AllocErr> {
        // Calculate the order of the free list to push a new frame to.
        // TODO: can we assume the max size is always equal to the frame size?
        //       if so, we can just always use the highest order...
        let layout = Layout::from_size_align(F::FRAME_SIZE, 1)
            .ok_or(AllocErr::Unsupported {
                details: "Unsupported layout for max FRAME_SIZE \
                         (this shouldn't happen)!"
            })?;
        let order = self.block_order(&layout)?;
        let mut new_frame = self.frames.alloc()?;
        let frame_ptr: *mut u8 = &mut new_frame as *mut F::Frame as *mut _;
        self.push_block(frame_ptr, order);
        Ok(())

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
