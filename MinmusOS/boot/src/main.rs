// Project Name:  MinmusOS
// File Name:     main.rs
// File Function: The entry of boot
// Author:        Jishen Lin
// License:       MIT License

#![no_std]
#![no_main]

mod disk;

use core::arch::asm;
use core::arch::global_asm;
use core::panic::PanicInfo;
use disk::DiskReader;

const BOOTLOADER_LBA: u64 = 2048;
const BOOTLOADER_SIZE: u16 = 64;

global_asm!(include_str!("boot.asm"));

extern "C" {
    static _bootloader_start: u16;
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    unsafe {
        asm!("mov ah, 0x00", "mov al, 0x03", "int 0x10");
    }
    print("[INFO] Booting MinmusOS...\r\n\0");
    print("[INFO] Loading Bootloader...\r\n\0");
    let bootloader_start: *const u16 = unsafe { &_bootloader_start };
    let target = bootloader_start as u16;
    let mut disk = DiskReader::new(BOOTLOADER_LBA, target);
    disk.read_sectors(BOOTLOADER_SIZE);
    unsafe {
        asm!("jmp {0:x}", in(reg) bootloader_start as u16);
    }
    loop {}
}

fn print(message: &str) {
    unsafe {
        asm!(
        "mov si, {0:x}",
        "2:",
        "lodsb",
        "or al, al",
        "jz 3f",
        "mov ah, 0x0e",
        "mov bh, 0",
        "int 0x10",
        "jmp 2b",
        "3:",
        in(reg) message.as_ptr(),
        );
    }
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    print("[ERROR] Failed to Load Bootloader!\r\n\0");
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}