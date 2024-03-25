//! Integration with Rust global memory allocator.
//!
//! This module provides [`use_rust_global_allocator`] function to setup the
//! underlying DataStax C++ driver for Apache Cassandra to use Rust global
//! memory allocator instead of its own memory management functions.
//!
//! The driver also uses this allocator for any third-party libraries that
//! allows to set custom memory management functions.
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

use crate::ffi::cass_alloc_set_functions;
use crate::DriverError;

/// Configures the underlying DataStax C++ driver for Apache Cassandra to use
/// Rust global memory allocator instead of its own memory management functions.
pub fn use_rust_global_allocator() -> Result<(), DriverError> {
    unsafe {
        cass_alloc_set_functions(
            Some(rust_global_allocator_alloc),
            Some(rust_global_allocator_realloc),
            Some(rust_global_allocator_free),
        );
    }

    Ok(())
}

unsafe extern "C" fn rust_global_allocator_alloc(size: usize) -> *mut c_void {
    // TODO: handle zero-sized allocations
    let layout =
        Layout::from_size_align(size + LAYOUT_DATA_SIZE, DEFAULT_ALIGNMENT)
            .expect("invalid memory layout");

    let block_start = alloc(layout);

    if block_start.is_null() {
        return null_mut::<c_void>();
    }

    store_layout(layout, block_start) as *mut c_void
}

unsafe extern "C" fn rust_global_allocator_realloc(
    ptr: *mut c_void,
    size: usize,
) -> *mut c_void {
    if ptr.is_null() {
        return rust_global_allocator_alloc(size);
    }

    // TODO: handle zero-sized allocations

    let new_size = size + LAYOUT_DATA_SIZE;

    let (block_start, layout) = restore_layout(ptr as *const u8);
    let new_layout = Layout::from_size_align(new_size, layout.align())
        .expect("invalid memory layout");

    let new_block_start = realloc(block_start, layout, new_size);

    if new_block_start.is_null() {
        return null_mut::<c_void>();
    }

    store_layout(new_layout, new_block_start) as *mut c_void
}

unsafe extern "C" fn rust_global_allocator_free(ptr: *mut c_void) {
    if ptr.is_null() {
        return;
    }

    let (block_start, layout) = restore_layout(ptr as *const u8);
    dealloc(block_start, layout);
}

// # The Problem
//
// The Rust allocator API requires a [`Layout`] struct to be passed into each of
// its functions. For example, to reallocate or free memory, you need to pass
// the pointer to the existing memory block and the layout used during its
// allocaton.
//
// This is a problem when interacting with C API which does not provide any
// means of storing additional allocation data.
//
// The driver's memory allocation functions is the example of such API, e.g.
//
// ```rust
// unsafe extern "C" fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void
// ```
//
// See? The memory reallocation function has the same signature as `realloc`
// from the standard C library and takes a pointer and the new size of the
// block.
//
// Now compare this to the Rust API:
//
// ```rust
// unsafe fn realloc(ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8
// ```
//
// This function has one additional parameter: the layout, I was talking about
// earlier. This MUST be the same layout used during the memory allocation.
//
// And here comes the problem: we have no information about the layout in the C
// API and thus cannot pass it to Rust.
//
// Oh, well, back to the old good memory tricks we used to use in C in the old
// days.
//
// # The Solution
//
// We need to store the layout used to allocate a memory block in the block
// itself, i.e. we need to allocate extra bytes, write the layout at the
// beginning of the block, and return the pointer to the first byte that follows
// the layout data. For example, when the driver requests 60 bytes of memory:
//
// ```text
//                      requested amount of bytes (60)
//                                     |
//    size_of::<usize> bytes (8)       |
//                 |                   |
//                 v                   v
//              +----+----------------------------------+
//      +------>| 68 |0000000000000000000000000000000000|
//      |       +----^----------------------------------+
// block size        |
//                   |
//    pointer we return to the caller
// ```
//
// Then, for Rust [`realloc`] and [`dealloc`] functions, we take the pointer,
// subtract the size of the allocation information, read the size of the block
// and reconstruct the layout structure. We don't need to store the alignment,
// as we always use the same alignment (C API does not provide alignment
// parameters at all anyway).

/// Stores the layout used during the allocation of the memory block to the
/// block itself and returns a pointer to the first byte after the layout data.
fn store_layout(layout: Layout, start: *mut u8) -> *mut u8 {
    let size_start = start as *mut usize;
    unsafe { copy_nonoverlapping(from_ref(&layout.size()), size_start, 1) };

    let align_start = unsafe { size_start.add(1) };
    unsafe {
        copy_nonoverlapping(from_ref(&DEFAULT_ALIGNMENT), align_start, 1)
    };

    unsafe { start.add(LAYOUT_DATA_SIZE) }
}

/// Restores the layout from the memory block and returns the layout and the
/// real start of the allocated memory block, i.e. the start of the allocation
/// data.
fn restore_layout(ptr: *const u8) -> (*mut u8, Layout) {
    let layout_start = unsafe { ptr.sub(LAYOUT_DATA_SIZE) };
    let size_start = layout_start as *const usize;
    let mut size = 0usize;
    unsafe { copy_nonoverlapping(size_start, from_mut(&mut size), 1) };

    let align_start = unsafe { size_start.add(1) };
    let mut align = 0usize;
    unsafe { copy_nonoverlapping(align_start, from_mut(&mut align), 1) };

    let layout =
        Layout::from_size_align(size, align).expect("invalid memory layout");

    (layout_start as *mut u8, layout)
}

/// The default alignment for memory allocation requests coming from C.
const DEFAULT_ALIGNMENT: usize = 1;
/// The size of the additional memory we use to store internal allocation
/// data which consists of the block size and alignment.
const LAYOUT_DATA_SIZE: usize = size_of::<usize>() + size_of::<usize>();
