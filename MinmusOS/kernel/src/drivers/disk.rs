// Project Name:  MinmusOS
// File Name:     disk.rs
// File Function: Disk driver
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;

const DATA_REGISTER: u16 = 0x1F0;
const SECTOR_COUNT_REGISTER: u16 = 0x1F2;
const LBA_LOW_REGISTER: u16 = 0x1F3;
const LBA_MID_REGISTER: u16 = 0x1F4;
const LBA_HIGH_REGISTER: u16 = 0x1F5;
const DRIVE_REGISTER: u16 = 0x1F6;
const STATUS_COMMAND_REGISTER: u16 = 0x1F7;
const READ_COMMAND: u8 = 0x20;
const STATUS_BSY: u8 = 0b10000000;
const STATUS_RDY: u8 = 0b01000000;

pub static mut DISK: Disk = Disk {
    enabled: false
};

pub struct Disk {
    pub enabled: bool,
}

impl Disk {
    pub fn read<T>(&self, target: *mut T, lba: u64, sectors: u16) {
        if !self.enabled {
            lib::println!("[ERROR] Failed to Read Disk!");
            return;
        }

        while self.is_busy() {}

        unsafe {
            asm!("out dx, al", in("dx") 0x3F6, in("al") 0b00000010u8);
            asm!("out dx, al", in("dx") SECTOR_COUNT_REGISTER, in("al") sectors as u8);
            asm!("out dx, al", in("dx") LBA_LOW_REGISTER, in("al") lba as u8);
            asm!("out dx, al", in("dx") LBA_MID_REGISTER, in("al") (lba >> 8) as u8);
            asm!("out dx, al", in("dx") LBA_HIGH_REGISTER, in("al") (lba >> 16) as u8);
            asm!("out dx, al", in("dx") DRIVE_REGISTER, in("al") (0xE0 | ((lba >> 24) & 0xF)) as u8);
            asm!("out dx, al", in("dx") STATUS_COMMAND_REGISTER, in("al") READ_COMMAND);
        }

        let mut sectors_left = sectors;
        let mut target_pointer = target;

        while sectors_left > 0 {
            for _i in 0..128 {
                while self.is_busy() {}
                while !self.is_ready() {}
                let buffer: u32;
                unsafe {
                    asm!("in eax, dx", out("eax") buffer, in("dx") DATA_REGISTER);
                    core::ptr::write_unaligned(target_pointer as *mut u32, buffer);
                    target_pointer = target_pointer.byte_add(4);
                }
            }
            sectors_left -= 1;
        }

        self.reset();
    }

    pub fn is_busy(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }
        (status & STATUS_BSY) != 0
    }

    pub fn is_ready(&self) -> bool {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }
        (status & STATUS_RDY) != 0
    }

    pub fn check(&mut self) {
        let status: u8;
        unsafe {
            asm!("in al, dx", out("al") status, in("dx") STATUS_COMMAND_REGISTER);
        }
        if status != 0 && status != 0xFF {
            self.enabled = true;
            lib::println!("[INFO] ATA Disk Driver is Working. Status Register: 0x{:X}", status);
        } else {
            self.enabled = false;
            lib::println!("[ERROR] ATA Disk Driver is not Working! Status Register: 0x{:X}", status);
        }
    }

    pub fn reset(&self) {
        unsafe {
            asm!("out dx, al", in("dx") 0x3F6, in("al") 0b00000110u8);
            asm!("out dx, al", in("dx") 0x3F6, in("al") 0b00000010u8);
        }
    }
}