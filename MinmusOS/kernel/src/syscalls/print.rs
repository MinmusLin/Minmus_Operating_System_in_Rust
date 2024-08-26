// Project Name:  MinmusOS
// File Name:     print.rs
// File Function: Printer
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;

const VGA_WIDTH: u16 = 80;
const VGA_HEIGHT: u16 = 25;
const VGA_START: u32 = 0x000B8000;
#[allow(dead_code)]
pub const COLOR_BLACK: u8 = 0x0;
#[allow(dead_code)]
pub const COLOR_BLUE: u8 = 0x1;
#[allow(dead_code)]
pub const COLOR_GREEN: u8 = 0x2;
#[allow(dead_code)]
pub const COLOR_CYAN: u8 = 0x3;
#[allow(dead_code)]
pub const COLOR_RED: u8 = 0x4;
#[allow(dead_code)]
pub const COLOR_MAGENTA: u8 = 0x5;
#[allow(dead_code)]
pub const COLOR_YELLOW: u8 = 0x6;
#[allow(dead_code)]
pub const COLOR_WHITE: u8 = 0x7;
#[allow(dead_code)]
pub const COLOR_LIGHT_BLACK: u8 = 0x8;
#[allow(dead_code)]
pub const COLOR_LIGHT_BLUE: u8 = 0x9;
#[allow(dead_code)]
pub const COLOR_LIGHT_GREEN: u8 = 0xA;
#[allow(dead_code)]
pub const COLOR_LIGHT_CYAN: u8 = 0xB;
#[allow(dead_code)]
pub const COLOR_LIGHT_RED: u8 = 0xC;
#[allow(dead_code)]
pub const COLOR_LIGHT_MAGENTA: u8 = 0xD;
#[allow(dead_code)]
pub const COLOR_LIGHT_YELLOW: u8 = 0xE;
#[allow(dead_code)]
pub const COLOR_LIGHT_WHITE: u8 = 0xF;

pub static mut PRINTER: Printer = Printer {
    x: 0,
    y: 0,
    fg_color: COLOR_WHITE,
    bg_color: COLOR_BLACK,
};

pub struct Printer {
    x: u16,
    y: u16,
    fg_color: u8,
    bg_color: u8,
}

impl Printer {
    pub fn printc(&mut self, c: char) {
        if c == '\n' {
            self.new_line();
            return;
        }
        let target: *mut u8 = (VGA_START + ((self.y * VGA_WIDTH + self.x) * 2) as u32) as *mut u8;
        unsafe {
            if self.y >= VGA_HEIGHT - 1 && self.x >= VGA_WIDTH - 1 {
                *target = c as u8;
                *target.byte_add(1) = self.bg_color << 4 | self.fg_color;
                self.scroll();
                self.x = 0;
            } else {
                *target = c as u8;
                *target.byte_add(1) = self.bg_color << 4 | self.fg_color;
                self.x += 1;
                if self.x >= VGA_WIDTH {
                    self.x = 0;
                    self.y += 1;
                }
            }
        }
        self.set_cursor_position();
    }

    pub fn prints(&mut self, s: &str) {
        let cursor: (u16, u16) = self.get_cursor_position();
        self.x = cursor.0;
        self.y = cursor.1;
        for c in s.chars() {
            self.printc(c);
        }
        self.set_cursor_position();
    }

    pub fn delete(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        } else if self.y > 0 {
            self.y -= 1;
            self.x = VGA_WIDTH - 1;
        } else {
            return;
        }
        let target: *mut u8 = (VGA_START + ((self.y * VGA_WIDTH + self.x) * 2) as u32) as *mut u8;
        unsafe {
            *target = b' ' as u8;
            *target.byte_add(1) = self.bg_color << 4 | self.fg_color;
        }
        self.set_cursor_position();
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        let mut index: u16 = 0;
        unsafe {
            asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Fu8);
            let mut a: u8;
            asm!("in al, dx", out("al") a, in("dx") 0x3D5);
            index |= a as u16;
            asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Eu8);
            let b: u8;
            asm!("in al, dx", out("al") b, in("dx") 0x3D5);
            index |= (b as u16) << 8;
        }
        (index % VGA_WIDTH, index / VGA_WIDTH)
    }

    pub fn set_cursor_position(&self) {
        let index: u16 = self.y * VGA_WIDTH + self.x;
        unsafe {
            asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Fu8);
            asm!("out dx, al", in("dx") 0x3D5u16, in("al") (index & 0xFF) as u8);
            asm!("out dx, al", in("dx") 0x3D4u16, in("al") 0x0Eu8);
            asm!("out dx, al", in("dx") 0x3D5u16, in("al") ((index >> 8) & 0xFF) as u8);
        }
    }

    pub fn scroll(&mut self) {
        for i in 0..VGA_HEIGHT {
            for j in (VGA_WIDTH * i)..((VGA_WIDTH * i) + VGA_WIDTH) {
                let new: *mut u8 = (VGA_START + (j * 2) as u32) as *mut u8;
                let old: *const u8 = (VGA_START + ((j + VGA_WIDTH) * 2) as u32) as *const u8;
                unsafe {
                    *new = *old;
                    *new.byte_add(1) = *old.byte_add(1);
                }
            }
        }
    }

    pub fn set_colors(&mut self, fg_color: u8, bg_color: u8) {
        self.fg_color = fg_color;
        self.bg_color = bg_color;
    }

    pub fn reset_colors(&mut self) {
        self.set_colors(COLOR_WHITE, COLOR_BLACK)
    }

    pub fn new_line(&mut self) {
        if self.y == VGA_HEIGHT - 1 {
            self.scroll();
        } else {
            self.y += 1;
        }
        self.x = 0;
        self.set_cursor_position();
    }

    pub fn clear(&mut self) {
        self.x = 0;
        self.y = 0;
        for i in 0..(VGA_WIDTH * VGA_HEIGHT) {
            let target: *mut u8 = (VGA_START + (i * 2) as u32) as *mut u8;
            unsafe {
                *target = b' ' as u8;
                *target.byte_add(1) = self.bg_color << 4 | self.fg_color;
            }
        }
        self.set_cursor_position();
    }
}