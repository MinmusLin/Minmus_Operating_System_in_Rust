// Project Name:  MinmusOS
// File Name:     main.rs
// File Function: The entry of kernel
// Author:        Jishen Lin
// License:       MIT License

#![no_std]
#![no_main]
#![feature(naked_functions)]

extern crate alloc;

mod drivers;
mod filesystem;
mod interrupts;
mod memory;
mod multitasking;
mod shell;
mod syscalls;
mod timer;

use core::arch::asm;
use core::panic::PanicInfo;
use drivers::disk::DISK;
use drivers::pic::PICS;
use interrupts::idt::IDT;
use memory::allocator::Allocator;
use memory::paging::PAGING;
use shell::shell::SHELL;
use syscalls::print::PRINTER;
use filesystem::fat::FAT;
use multitasking::task::TASK_MANAGER;
use lib;
use crate::syscalls::print::{COLOR_BLACK, COLOR_LIGHT_WHITE, COLOR_LIGHT_YELLOW, COLOR_RED};

#[global_allocator]
static ALLOCATOR: Allocator = Allocator::new();

const KERNEL_START: u32 = 0x00100000;
const KERNEL_SIZE: u32 = 0x00100000;
const STACK_SIZE: u32 = 0x00100000;
const STACK_START: u32 = KERNEL_START + KERNEL_SIZE + STACK_SIZE;

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() -> ! {
    unsafe {
        PRINTER.prints("[INFO] Initializing Stack...\n");
        asm!("mov esp, {}", in(reg) STACK_START);

        PRINTER.prints("[INFO] Initializing Paging...\n");
        PAGING.identity();
        PAGING.enable();
        asm!("xchg bx, bx");

        PRINTER.prints("[INFO] Loading Interrupt Descriptor Table...\n");
        IDT.init();
        IDT.add_exceptions();
        IDT.add(
            interrupts::timer::TIMER_INT as usize,
            interrupts::timer::timer as u32,
        );
        IDT.add(
            syscalls::handler::SYSCALL_INT as usize,
            syscalls::handler::syscall as u32,
        );
        IDT.add(
            drivers::keyboard::KEYBOARD_INT as usize,
            drivers::keyboard::keyboard as u32,
        );
        IDT.load();

        PRINTER.prints("[INFO] Initializing Programmable Interrupt Controllers...\n");
        PICS.init();

        PRINTER.prints("[INFO] Initializing FAT16 File System...\n");
        DISK.check();
        if DISK.enabled {
            let fat = FAT.acquire_mut();
            fat.load_header();
            fat.load_table();
            fat.load_entries();
            FAT.free();
        }

        PRINTER.prints("[INFO] Initializing Multitasking...\n");
        TASK_MANAGER.init();

        PRINTER.prints("[INFO] Initializing Shell...\n");
        print_info();
        SHELL.init();
        asm!("xchg bx, bx");
        asm!("sti");

        loop {}
    }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        PRINTER.set_colors(COLOR_LIGHT_WHITE, COLOR_RED);
        PRINTER.clear();
    }
    lib::println!();
    lib::println!("  ========== MinmusOS v1.0 ==========                            By Jishen Lin");
    lib::println!();
    lib::println!("  KERNEL PANIC!");
    lib::println!();
    lib::println!("  MinmusOS has encountered a panic and the system has been halted.");
    lib::println!();
    lib::println!();
    lib::println!();
    lib::println!();
    lib::println!();
    lib::println!();
    lib::println!("  PANIC DESCRIPTION:");
    lib::println!();
    lib::println!("    {}", info.message());
    lib::println!();
    lib::println!();
    lib::println!("  TECHNICAL INFORMATION:");
    lib::println!();
    lib::println!("    FILE : {}", info.location().unwrap().file());
    lib::println!("    LINE : {}", info.location().unwrap().line());
    lib::println!();
    lib::println!();
    lib::print!("  Please restart your computer. :)");
    loop {}
}

fn print_info() {
    unsafe {
        PRINTER.set_colors(COLOR_LIGHT_YELLOW, COLOR_BLACK);
    }
    lib::println!();
    lib::println!("  Welcome to MinmusOS v1.0!");
    lib::println!("  MIT License - Copyright (c) 2024 Jishen Lin");
    lib::println!();
    lib::println!("  Type \"help\" and press enter to show available commands.");
    lib::println!();
    unsafe {
        PRINTER.reset_colors();
    }
}