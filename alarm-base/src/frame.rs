//! Base types for page frame allocators.
use core::alloc::{Layout, AllocErr};
use ::AllocResult;
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

/// A fixed-size cache of three frames that can be used as a frame allocator
/// when a normal one is unavailable.
///
/// This will be used primarily during the kernel remapping, but it's also
/// useful for e.g. memory allocator testing.
#[derive(Debug)]
pub struct FrameCache<F>([Option<F>; 3]);

impl<F> FrameCache<F> {

    /// Construct a new `FrameCache` from three provided frames.
    pub fn from_frames(f1: F, f2: F, f3: F) -> Self {
        FrameCache([Some(f1), Some(f2), Some(f3)])
    }

}

impl<F> FrameCache<F>
where
    F: Page,
{

    /// Construct a new `FrameCache` with frames allocated
    /// by the provided `Allocator`.
    pub fn from_alloc<A>(alloc: &mut A) -> Self
    where A: Allocator<Frame=F> {
        unsafe {
            let frames = [ alloc.alloc().ok()
                         , alloc.alloc().ok()
                         , alloc.alloc().ok() ];
            FrameCache(frames)
        }
    }
}

unsafe impl<F> Allocator for FrameCache<F>
where
    F: Page,
{

    type Frame = F;

    const FRAME_SIZE: usize = <F as Page>::SIZE;

    unsafe fn alloc(&mut self) -> AllocResult<Self::Frame> {
        self.0.iter_mut()
            .find(|frame| frame.is_some())
            .and_then(|frame| frame.take())
            // .map(|frame| { trace!("frameCache: alloced {:?}", &frame); frame})
            .ok_or(AllocErr::Exhausted {
                request: Layout::from_size_align(
                    Self::FRAME_SIZE, Self::FRAME_SIZE).unwrap()
            })
    }

    unsafe fn dealloc(&mut self, frame: Self::Frame) -> AllocResult<()> {
        self.0.iter_mut()
            .find(|slot| slot.is_none())
            .and_then(|slot| { *slot = Some(frame); Some(()) })
            .ok_or(AllocErr::Unsupported {
                details: "FrameCache can only hold three frames!"
            })
    }

}
