// Project Name:  MinmusOS
// File Name:     print.rs
// File Function: Printer
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;
use core::fmt;

pub static mut PRINTER: Printer = Printer {};

pub struct Printer {}

impl fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.prints(s);
        Ok(())
    }
}

impl Printer {
    pub fn printc(&self, c: char) {
        unsafe {
            asm!(
            "int 0x10",
            in("al") c as u8,
            in("ah") 0x0eu8,
            in("bx") 0u16,
            );
        }
    }

    pub fn prints(&self, s: &str) {
        for c in s.chars() {
            self.printc(c);
        }
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        unsafe {
            asm!("int 0x10", in("ax") 0x0003u16);
        }
    }
}

#[macro_export]
macro_rules! clear {
    () => {
        $crate::print::_clear()
    };
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\r\n"));
    ($($arg:tt)*) => ($crate::print!("{}\r\n", format_args!($($arg)*)));
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}

pub fn _clear() {
    unsafe {
        PRINTER.clear();
    }
}