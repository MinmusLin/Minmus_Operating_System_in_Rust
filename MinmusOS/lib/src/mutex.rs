// Project Name:  MinmusOS
// File Name:     mutex.rs
// File Function: Mutex utils
// Author:        Jishen Lin
// License:       MIT License

use core::sync::atomic::{AtomicBool, Ordering};

pub struct Mutex<T> {
    target: T,
    free: AtomicBool,
}

impl<T> Mutex<T> {
    pub const fn new(value: T) -> Self {
        Self {
            target: value,
            free: AtomicBool::new(true),
        }
    }

    pub fn acquire_mut(&mut self) -> &mut T {
        while !self.free.load(Ordering::SeqCst) {}
        self.free.store(false, Ordering::SeqCst);
        &mut self.target
    }

    pub fn acquire(&mut self) -> &T {
        while !self.free.load(Ordering::SeqCst) {}
        self.free.store(false, Ordering::SeqCst);
        &self.target
    }

    pub fn free(&self) {
        self.free.store(true, Ordering::SeqCst);
    }
}

impl<T> Drop for Mutex<T> {
    fn drop(&mut self) {
        self.free = AtomicBool::from(true);
    }
}