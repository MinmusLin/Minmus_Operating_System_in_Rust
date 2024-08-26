// Project Name:  MinmusOS
// File Name:     allocator.rs
// File Function: Memory allocator
// Author:        Jishen Lin
// License:       MIT License

use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicPtr, Ordering};

#[derive(Copy, Clone, Debug)]
struct Block {
    size: usize,
    next: Option<NonNull<Block>>,
}

pub struct Allocator {
    head: AtomicPtr<Block>,
}

impl Allocator {
    pub const fn new() -> Self {
        Allocator {
            head: AtomicPtr::new(ptr::null_mut())
        }
    }

    #[allow(dead_code)]
    pub unsafe fn init(&self, heap_start: usize, heap_size: usize) {
        let block = Block {
            size: heap_size,
            next: None,
        };
        let block_ptr: *mut Block = heap_start as *mut Block;
        ptr::write(block_ptr, block);
        self.head.store(block_ptr, Ordering::SeqCst);
    }
}

unsafe impl GlobalAlloc for Allocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut current: *mut Block = self.head.load(Ordering::SeqCst);
        let alloc_size: usize = layout.size().max(layout.align());
        while !current.is_null() {
            let current_block: &mut Block = &mut *current;
            if current_block.size >= alloc_size {
                self.head.store(current_block.next.map_or(ptr::null_mut(), |b| b.as_ptr()), Ordering::SeqCst);
                return current as *mut u8;
            }
            current = current_block.next.map_or(ptr::null_mut(), |b| b.as_ptr());
        }
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let mut new_block = Block {
            size: layout.size(),
            next: None,
        };
        new_block.next = NonNull::new(self.head.load(Ordering::SeqCst));
        let new_block_ptr: *mut Block = ptr as *mut Block;
        ptr::write(new_block_ptr, new_block);
        self.head.store(new_block_ptr, Ordering::SeqCst);
    }
}