// Project Name:  MinmusOS
// File Name:     main.rs
// File Function: The entry of bootloader
// Author:        Jishen Lin
// License:       MIT License

#![no_std]
#![no_main]
#[macro_use]

mod print;
mod disk;
mod gdt;

use core::arch::asm;
use core::panic::PanicInfo;
use disk::DISK;
use gdt::GDT;

const KERNEL_LBA: u64 = 4096;
const KERNEL_SIZE: u16 = 2048;
const KERNEL_BUFFER: u16 = 0xBE00;
const KERNEL_TARGET: u32 = 0x00100000;

fn unreal_mode() {
    let ds: u16;
    let ss: u16;
    unsafe {
        asm!("mov {0:x}, ds", out(reg) ds);
        asm!("mov {0:x}, ss", out(reg) ss);
    }

    GDT.load();

    unsafe {
        let mut cr0: u32;
        asm!("mov {0:e}, cr0", out(reg) cr0);
        let cr0_protected = cr0 | 1;
        asm!("mov cr0, {0:e}", in(reg) cr0_protected);
        asm!("mov {0:x}, 0x10", "mov ds, {0:x}", "mov ss, {0:x}", out(reg) _);
        asm!("mov cr0, {0:e}", in(reg) cr0);
        asm!("mov ds, {0:x}", in(reg) ds);
        asm!("mov ss, {0:x}", in(reg) ss);
    }
}

fn protected_mode() {
    unsafe {
        asm!("mov eax, cr0", "or al, 1", "mov cr0, eax");
        asm!("push {0:e}", in(reg) KERNEL_TARGET);
        asm!("ljmp $0x8, $2f", "2:", options(att_syntax));
        asm!(
        ".code32",
        "mov {0:e}, 0x10",
        "mov ds, {0:e}",
        "mov es, {0:e}",
        "mov ss, {0:e}",
        "pop {1:e}",
        "call {1:e}",
        out(reg) _,
        in(reg) KERNEL_TARGET,
        );
    }
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    println!("[INFO] Switching to Unreal Mode...");
    unreal_mode();
    println!("[INFO] Loading Kernel...");
    unsafe {
        DISK.init(KERNEL_LBA, KERNEL_BUFFER);
        DISK.read_sectors(KERNEL_SIZE, KERNEL_TARGET);
    }
    println!("[INFO] Loading Global Descriptor Table...");
    GDT.load();
    println!("[INFO] Switching to Protected Mode...");
    protected_mode();
    loop {}
}

#[no_mangle]
pub extern "C" fn fail() -> ! {
    println!("[ERROR] Failed to Load Bootloader!");
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("[PANIC] Info: {}", info);
    loop {}
}