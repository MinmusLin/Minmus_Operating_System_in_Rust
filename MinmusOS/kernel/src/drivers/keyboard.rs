// Project Name:  MinmusOS
// File Name:     keyboard.rs
// File Function: Keyboard driver
// Author:        Jishen Lin
// License:       MIT License

use crate::drivers::pic::PICS;
use crate::shell::shell::SHELL;
use core::arch::asm;

pub const KEYBOARD_INT: u8 = 33;

pub struct Keyboard {
    left_shift: bool,
    right_shift: bool,
    left_ctrl: bool,
    right_ctrl: bool,
    left_alt: bool,
    right_alt: bool,
    caps_lock: bool,
}

pub static mut KEYBOARD: Keyboard = Keyboard {
    left_shift: false,
    right_shift: false,
    left_ctrl: false,
    right_ctrl: false,
    left_alt: false,
    right_alt: false,
    caps_lock: false,
};

#[derive(Copy, Clone, Debug)]
struct KeyMap {
    scancode: u8,
    normal: char,
    shifted: char,
}

static KEYMAP: &[KeyMap] = &[
    KeyMap { scancode: 0x02, normal: '1', shifted: '!' },
    KeyMap { scancode: 0x03, normal: '2', shifted: '@' },
    KeyMap { scancode: 0x04, normal: '3', shifted: '#' },
    KeyMap { scancode: 0x05, normal: '4', shifted: '$' },
    KeyMap { scancode: 0x06, normal: '5', shifted: '%' },
    KeyMap { scancode: 0x07, normal: '6', shifted: '^' },
    KeyMap { scancode: 0x08, normal: '7', shifted: '&' },
    KeyMap { scancode: 0x09, normal: '8', shifted: '*' },
    KeyMap { scancode: 0x0A, normal: '9', shifted: '(' },
    KeyMap { scancode: 0x0B, normal: '0', shifted: ')' },
    KeyMap { scancode: 0x0C, normal: '-', shifted: '_' },
    KeyMap { scancode: 0x0D, normal: '=', shifted: '+' },
    KeyMap { scancode: 0x10, normal: 'q', shifted: 'Q' },
    KeyMap { scancode: 0x11, normal: 'w', shifted: 'W' },
    KeyMap { scancode: 0x12, normal: 'e', shifted: 'E' },
    KeyMap { scancode: 0x13, normal: 'r', shifted: 'R' },
    KeyMap { scancode: 0x14, normal: 't', shifted: 'T' },
    KeyMap { scancode: 0x15, normal: 'y', shifted: 'Y' },
    KeyMap { scancode: 0x16, normal: 'u', shifted: 'U' },
    KeyMap { scancode: 0x17, normal: 'i', shifted: 'I' },
    KeyMap { scancode: 0x18, normal: 'o', shifted: 'O' },
    KeyMap { scancode: 0x19, normal: 'p', shifted: 'P' },
    KeyMap { scancode: 0x1E, normal: 'a', shifted: 'A' },
    KeyMap { scancode: 0x1F, normal: 's', shifted: 'S' },
    KeyMap { scancode: 0x20, normal: 'd', shifted: 'D' },
    KeyMap { scancode: 0x21, normal: 'f', shifted: 'F' },
    KeyMap { scancode: 0x22, normal: 'g', shifted: 'G' },
    KeyMap { scancode: 0x23, normal: 'h', shifted: 'H' },
    KeyMap { scancode: 0x24, normal: 'j', shifted: 'J' },
    KeyMap { scancode: 0x25, normal: 'k', shifted: 'K' },
    KeyMap { scancode: 0x26, normal: 'l', shifted: 'L' },
    KeyMap { scancode: 0x2C, normal: 'z', shifted: 'Z' },
    KeyMap { scancode: 0x2D, normal: 'x', shifted: 'X' },
    KeyMap { scancode: 0x2E, normal: 'c', shifted: 'C' },
    KeyMap { scancode: 0x2F, normal: 'v', shifted: 'V' },
    KeyMap { scancode: 0x30, normal: 'b', shifted: 'B' },
    KeyMap { scancode: 0x31, normal: 'n', shifted: 'N' },
    KeyMap { scancode: 0x32, normal: 'm', shifted: 'M' },
    KeyMap { scancode: 0x1A, normal: '[', shifted: '{' },
    KeyMap { scancode: 0x1B, normal: ']', shifted: '}' },
    KeyMap { scancode: 0x2B, normal: '\\', shifted: '|' },
    KeyMap { scancode: 0x27, normal: ';', shifted: ':' },
    KeyMap { scancode: 0x28, normal: '\'', shifted: '"' },
    KeyMap { scancode: 0x33, normal: ',', shifted: '<' },
    KeyMap { scancode: 0x34, normal: '.', shifted: '>' },
    KeyMap { scancode: 0x35, normal: '/', shifted: '?' },
    KeyMap { scancode: 0x29, normal: '`', shifted: '~' },
    KeyMap { scancode: 0x39, normal: ' ', shifted: ' ' },
];

#[naked]
pub extern "C" fn keyboard() {
    unsafe {
        asm!(
        "push 0x6D6E6276",
        "push 0x63787A6C",
        "push 0x6B6A6867",
        "push 0x66647361",
        "push 0x706F6975",
        "push 0x79747265",
        "push 0x77713039",
        "push 0x38373635",
        "push 0x34333231",
        "call keyboard_handler",
        "add esp, 36",
        "iretd",
        options(noreturn),
        );
    }
}

fn scancode_to_char(scancode: u8) -> char {
    let shift_pressed: bool = unsafe { KEYBOARD.left_shift || KEYBOARD.right_shift };
    let caps_lock: bool = unsafe { KEYBOARD.caps_lock };
    for key in KEYMAP.iter() {
        if key.scancode == scancode {
            let mut character = if shift_pressed {
                key.shifted
            } else {
                key.normal
            };
            if character.is_ascii_alphabetic() {
                if caps_lock && !shift_pressed || !caps_lock && shift_pressed {
                    character = character.to_ascii_uppercase();
                } else {
                    character = character.to_ascii_lowercase();
                }
            }
            return character;
        }
    }
    '\0'
}

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn keyboard_handler() {
    static mut E0_PREFIX: bool = false;
    let mut scancode: u8;
    unsafe {
        asm!("in al, dx", out("al") scancode, in("dx") 0x60u16);
        if scancode == 0xE0 {
            asm!("in al, dx", out("al") scancode, in("dx") 0x60u16);
            E0_PREFIX = true;
        }
        // lib::print!("[0x{:X}]", scancode);
        PICS.end_interrupt(KEYBOARD_INT);
        if E0_PREFIX {
            match scancode {
                0x1D => KEYBOARD.right_ctrl = true,
                0x9D => KEYBOARD.right_ctrl = false,
                0x38 => KEYBOARD.right_alt = true,
                0xB8 => KEYBOARD.right_alt = false,
                _ => {}
            }
            E0_PREFIX = false;
        } else {
            match scancode {
                0x2A => KEYBOARD.left_shift = true,
                0xAA => KEYBOARD.left_shift = false,
                0x36 => KEYBOARD.right_shift = true,
                0xB6 => KEYBOARD.right_shift = false,
                0x1D => KEYBOARD.left_ctrl = true,
                0x9D => KEYBOARD.left_ctrl = false,
                0x38 => KEYBOARD.left_alt = true,
                0xB8 => KEYBOARD.left_alt = false,
                0x3A => {
                    KEYBOARD.caps_lock = !KEYBOARD.caps_lock;
                    return;
                }
                0x0E => {
                    SHELL.backspace();
                    return;
                }
                0x1C => {
                    SHELL.enter();
                    return;
                }
                _ => {}
            }
        }
        let key: char = scancode_to_char(scancode);
        if key != '\0' {
            SHELL.add(key);
        }
    }
}