// Project Name:  MinmusOS
// File Name:     color.rs
// File Function: The implementation of the command color
// Author:        Jishen Lin
// License:       MIT License

use crate::syscalls::print::PRINTER;

pub fn color() {
    for fg in 0..16 {
        for bg in 0..16 {
            unsafe {
                PRINTER.set_colors(fg, bg);
            }
            lib::print!(" {:X}{:X} ", fg, bg);
        }
        unsafe {
            PRINTER.reset_colors();
        }
        lib::print!("    0x0{:X}", fg);
        unsafe {
            PRINTER.reset_colors();
        }
        lib::print!("  ");
        unsafe {
            PRINTER.set_colors(fg, fg);
        }
        lib::println!("    ");
    }
    unsafe {
        PRINTER.reset_colors();
    }
}