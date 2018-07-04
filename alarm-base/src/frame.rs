//! Base types for page frame allocators.
use alloc::allocator::AllocErr;
use hal9000::mem::Page;

/// An allocator that provides page frames.
pub unsafe trait Allocator {
    /// Architecture-dependent size of a physical page.
    const FRAME_SIZE: usize = <Self::Frame as Page>::SIZE;

    /// Type representing frames provided by this allocator.
    ///
    /// A `Frame` must either be a pointer to a contiguous block of `FRAME_SIZE`
    /// bytes, or be a handle that may be converted into such a pointer.
    type Frame: Page;

    /// Returns a new `Frame`.
    unsafe fn alloc(&mut self) -> Result<Self::Frame, AllocErr>;

    /// Deallocate a `Frame`.
    ///
    /// # Unsafety
    /// This function is unsafe because undefined behaviour may result if the
    /// given `frame` was not originally allocated by this `Allocator`.
    unsafe fn dealloc(&mut self, frame: Self::Frame) -> Result<(), AllocErr>;

    // TODO: alloc_range/dealloc_range; requires an architecture-independent
    //       way of representing frame ranges.
}
