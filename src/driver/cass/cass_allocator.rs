use std::alloc::{
    alloc,
    dealloc,
    realloc,
    Layout,
};
use std::ffi::c_void;
use std::mem::size_of;
use std::ptr::{
    copy_nonoverlapping,
    from_mut,
    from_ref,
    null_mut,
};

use crate::driver::cass::CassError;
use crate::driver::ffi::cass_alloc_set_functions;

/// Configures the underlying DataStax C++ driver for Apache Cassandra to use
/// Rust global memory allocator instead of its own memory management functions.
pub fn cass_use_rust_global_allocator() -> CassError {
    unsafe {
        cass_alloc_set_functions(
            Some(rust_global_allocator_alloc),
            Some(rust_global_allocator_realloc),
            Some(rust_global_allocator_free),
        );
    }

    CassError::Ok
}

unsafe extern "C" fn rust_global_allocator_alloc(size: usize) -> *mut c_void {
    MemoryBlock::alloc(size)
        .map(|m| m.requested_block_start() as *mut c_void)
        .unwrap_or(null_mut::<c_void>())
}

unsafe extern "C" fn rust_global_allocator_realloc(
    ptr: *mut c_void,
    size: usize,
) -> *mut c_void {
    MemoryBlock::from_raw(ptr)
        .and_then(|m| m.realloc(size))
        .map(|m| m.requested_block_start() as *mut c_void)
        .unwrap_or(null_mut::<c_void>())
}

unsafe extern "C" fn rust_global_allocator_free(ptr: *mut c_void) {
    if let Some(block) = MemoryBlock::from_raw(ptr) {
        block.dealloc();
    }
}

/// Allocates, deallocates, and frees memory blocks while storing the block size
/// inside them.
///
/// # The Problem
///
/// The Rust allocator API requires a [`Layout`] struct to be passed into each
/// of its functions. For example, to reallocate or free memory, you need to
/// pass the pointer to the existing memory block and the layout used during its
/// allocaton.
///
/// This is a problem when interacting with C API which does not provide any
/// means of storing additional allocation data.
///
/// The driver's memory allocation functions is the example of such API, e.g.
///
/// ```rust
/// unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void
/// ```
///
/// See? The memory reallocation function has the same signature as `realloc`
/// from the standard C library and takes a pointer and the new size of the
/// block.
///
/// Now compare this to the Rust API:
///
/// ```rust
/// unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
/// ```
///
/// This function has one additional parameter: the layout, I was talking about
/// earlier. This MUST be the same layout used during the memory allocation.
///
/// And here comes the problem: we have no information about the layout in the C
/// API and thus cannot pass it to Rust.
///
/// Oh, well, back to the old good memory tricks we used to use in C in the old
/// days.
///
/// # The Solution
///
/// We need to store the layout used to allocate a memory block in the block
/// itself, i.e. we need to allocate extra bytes, write the layout at the
/// beginning of the block, and return the pointer to the first byte that
/// follows the layout data. For example, when the driver requests 60 bytes of
/// memory:
///
/// ```text
///                      requested amount of bytes (60)
///                                     |
///    size_of::<usize> bytes (8)       |
///                 |                   |
///                 v                   v
///              +----+----------------------------------+
///      +------>| 68 |0000000000000000000000000000000000|
///      |       +----^----------------------------------+
/// block size        |
///                   |
///    pointer we return to the caller
/// ```
///
/// Then, for Rust [`realloc`] and [`dealloc`] functions, we take the pointer,
/// subtract the size of the allocation information, read the size of the block
/// and reconstruct the layout structure. We don't need to store the alignment,
/// as we always use the same alignment (C API does not provide alignment
/// parameters at all anyway).
struct MemoryBlock(*mut u8);

impl MemoryBlock {
    /// The default alignment for memory allocation requests coming from C.
    const ALIGNMENT: usize = 1;

    /// Tries to allocate a memory block of the given size.
    ///
    /// The `size` does not include the size of the additional memory we use to
    /// store internal allocation data.
    pub fn alloc(size: usize) -> Option<Self> {
        let alloc_data_size = size_of::<usize>();
        let whole_block_size = size + alloc_data_size;

        let layout = Self::make_layout(whole_block_size)?;
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return None;
        }

        Self::write_layout(ptr, whole_block_size);

        Some(Self(ptr))
    }

    /// Creates the memory block wrapper from the given pointer.
    ///
    /// The function is unsafe as it is impossible to check the block was
    /// previously allocated via [`MemoryBlock::alloc`].
    pub unsafe fn from_raw(ptr: *mut c_void) -> Option<Self> {
        if ptr.is_null() {
            None
        } else {
            Some(Self(ptr as *mut u8))
        }
    }

    /// Resizes this memory block to the given new size.
    pub fn realloc(self, new_size: usize) -> Option<Self> {
        let ptr = self.block_start();
        let layout = self.layout();
        let total_new_size = new_size + layout.size();

        let ptr = unsafe { realloc(ptr, layout, total_new_size) };
        if ptr.is_null() {
            return None;
        }

        Self::write_layout(ptr, total_new_size);

        Some(Self(ptr))
    }

    /// Deallocates the memory block.
    pub fn dealloc(self) {
        let layout = self.layout();
        let ptr = self.block_start();
        unsafe { dealloc(ptr, layout) };
    }

    /// Returns a pointer to the start of the whole block.
    pub fn block_start(&self) -> *mut u8 {
        self.0
    }

    /// Returns a pointer to the start of the block returned to the caller.
    pub fn requested_block_start(&self) -> *mut u8 {
        unsafe { self.0.byte_add(size_of::<usize>()) }
    }

    /// Returns the layout used during the allocation of this memory block.
    pub fn layout(&self) -> Layout {
        Self::read_layout(self.block_start()).expect(
            "the already allocated memory blocks must have a valid layout",
        )
    }

    fn read_layout(ptr: *const u8) -> Option<Layout> {
        let ptr = unsafe { ptr.byte_sub(size_of::<usize>()) } as *const usize;

        let mut size = 0usize;
        unsafe { copy_nonoverlapping(ptr, from_mut(&mut size), 1) };

        Self::make_layout(size)
    }

    fn write_layout(ptr: *mut u8, size: usize) {
        unsafe { copy_nonoverlapping(from_ref(&size), ptr as *mut usize, 1) };
    }

    fn make_layout(size: usize) -> Option<Layout> {
        Layout::from_size_align(size, Self::ALIGNMENT).ok()
    }
}
