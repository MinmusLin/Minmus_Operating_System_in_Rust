// Project Name:  MinmusOS
// File Name:     print.rs
// File Function: Print utils
// Author:        Jishen Lin
// License:       MIT License

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
    pub fn prints(&self, s: &str) {
        unsafe {
            let ptr: *const u8 = s.as_ptr();
            let len: usize = s.len();
            core::arch::asm!(
            "push eax",
            "push ebx",
            "push ecx",
            "int 0x80",
            "pop ecx",
            "pop ebx",
            "pop eax",
            in("eax") 0,
            in("ebx") ptr as u32,
            in("ecx") len as u32,
            );
        }
    }
}

pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        PRINTER.write_fmt(args).unwrap();
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => {
        unsafe {
            $crate::print::PRINTER.prints("\n");
        }
    };

    ($($arg:tt)*) => {
        $crate::print!("{}", format_args!($($arg)*));
        unsafe {
            $crate::print::PRINTER.prints("\n");
        }
    };
}