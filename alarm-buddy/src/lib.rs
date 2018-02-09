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
extern crate alloc;
extern crate alarm_base;
extern crate intruder_alarm;
extern crate spin;

use alarm_base::FrameAllocator;

use intruder_alarm::list::{List, Linked, Links};
use intruder_alarm::UnsafeRef;

pub type FreeList = List<FreeBlock, FreeBlock, UnsafeRef<FreeBlock>>;

/// A free block header.
pub type FreeBlock = Links<FreeBlock>;

/// A buddy-block allocator.
pub struct Heap<'a, F> {

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

    /// The underlying frame provider.
    frames: &'a mut F>,

}

impl<'a, F> Heap<'a, F> {

    /// Push a block of the given order.
    //
    // TODO: should this take a `Layout` instead?
    unsafe fn push_block(&mut self, ptr: *mut u8, order: usize) {
        unimplemented!()
    }

}



// ===== impl FreeBlock =====

impl Linked for FreeBlock {
    #[inline]
    fn links(&self) -> &Links<Self> {
        &self
    }

    #[inline]
    fn links_mut(&mut self) -> &mut Links<Self> {
        &mut self
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
