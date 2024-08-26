// Project Name:  MinmusOS
// File Name:     exceptions.rs
// File Function: CPU exceptions handlers
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;
use crate::syscalls::print::{COLOR_BLUE, COLOR_LIGHT_WHITE, PRINTER};

#[no_mangle]
pub extern "C" fn exception_handler(int: u32, eip: u32, cs: u32, eflags: u32) {
    unsafe {
        PRINTER.set_colors(COLOR_LIGHT_WHITE, COLOR_BLUE);
        PRINTER.clear();
    }
    lib::println!();
    lib::println!("  ========== MinmusOS v1.0 ==========                            By Jishen Lin");
    lib::println!();
    lib::println!("  SERIOUS KERNEL ERROR!");
    lib::println!();
    lib::println!("  MinmusOS has encountered a fatal error and the system has been halted.");
    lib::println!();
    lib::println!();
    lib::println!();
    lib::println!();
    lib::println!("  EXCEPTION DESCRIPTION:");
    lib::println!();
    match int {
        0x00 => {
            lib::println!("    DIVISION ERROR!");
        }
        0x01 => {
            lib::println!("    DEBUG EXCEPTION!");
        }
        0x02 => {
            lib::println!("    NMI INTERRUPT!");
        }
        0x03 => {
            lib::println!("    BREAKPOINT!");
        }
        0x04 => {
            lib::println!("    OVERFLOW!");
        }
        0x05 => {
            lib::println!("    BOUND RANGE EXCEEDED!");
        }
        0x06 => {
            lib::println!("    INVALID OPCODE (UNDEFINED OPCODE)!");
        }
        0x07 => {
            lib::println!("    DEVICE NOT AVAILABLE (NO MATH COPROCESSOR)!");
        }
        0x08 => {
            lib::println!("    DOUBLE FAULT!");
        }
        0x09 => {
            lib::println!("    COPROCESSOR SEGMENT OVERRUN!");
        }
        0x0A => {
            lib::println!("    INVALID TSS!");
        }
        0x0B => {
            lib::println!("    SEGMENT NOT PRESENT!");
        }
        0x0C => {
            lib::println!("    STACK-SEGMENT FAULT!");
        }
        0x0D => {
            lib::println!("    GENERAL PROTECTION!");
        }
        0x0E => {
            lib::println!("    PAGE FAULT!");
        }
        0x10 => {
            lib::println!("    x87 FPU FLOATING-POINT ERROR (MATH ERROR)!");
        }
        0x11 => {
            lib::println!("    ALIGNMENT CHECK!");
        }
        0x12 => {
            lib::println!("    MACHINE CHECK!");
        }
        0x13 => {
            lib::println!("    SIMD FLOATING-POINT EXCEPTION!");
        }
        0x14 => {
            lib::println!("    VIRTUALIZATION EXCEPTION!");
        }
        0x15 => {
            lib::println!("    CONTROL PROTECTION EXCEPTION!");
        }
        _ => {
            lib::println!("    EXCEPTION!");
        }
    }
    lib::println!();
    lib::println!();
    lib::println!("  TECHNICAL INFORMATION:");
    lib::println!();
    lib::println!("    ERROR_CODE      : {}", int);
    lib::println!("    INSTRUCTION_PTR : 0x{:X}", eip);
    lib::println!("    CODE_SEGMENT    : 0x{:X}", cs);
    lib::println!("    EXTENDED_FLAGS  : 0b{:b}", eflags);
    lib::println!();
    lib::println!();
    lib::print!("  Please restart your computer. :)");
    loop {}
}

#[naked]
pub extern "C" fn division_error() {
    unsafe {
        asm!(
        "push 0x00",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn debug_exception() {
    unsafe {
        asm!(
        "push 0x01",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn mni_interrupt() {
    unsafe {
        asm!(
        "push 0x02",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn breakpoint() {
    unsafe {
        asm!(
        "push 0x03",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn overflow() {
    unsafe {
        asm!(
        "push 0x04",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn bound_range_exceeded() {
    unsafe {
        asm!(
        "push 0x05",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn invalid_opcode() {
    unsafe {
        asm!(
        "push 0x06",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn device_not_available() {
    unsafe {
        asm!(
        "push 0x07",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn double_fault() {
    unsafe {
        asm!(
        "push 0x08",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn coprocessor_segment_overrun() {
    unsafe {
        asm!(
        "push 0x09",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn invalid_tss() {
    unsafe {
        asm!(
        "push 0x0A",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn segment_not_present() {
    unsafe {
        asm!(
        "push 0x0B",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn stack_segment_fault() {
    unsafe {
        asm!(
        "push 0x0C",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn general_protection() {
    unsafe {
        asm!(
        "push 0x0D",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn page_fault() {
    unsafe {
        asm!(
        "push 0x0E",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn x87_fpu_floating_point_error() {
    unsafe {
        asm!(
        "push 0x10",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn alignment_check() {
    unsafe {
        asm!(
        "push 0x11",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn machine_check() {
    unsafe {
        asm!(
        "push 0x12",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn simd_floating_point_exception() {
    unsafe {
        asm!(
        "push 0x13",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn virtualization_exception() {
    unsafe {
        asm!(
        "push 0x14",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn control_protection_exception() {
    unsafe {
        asm!(
        "push 0x15",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}

#[naked]
pub extern "C" fn generic_handler() {
    unsafe {
        asm!(
        "push 0xFF",
        "call exception_handler",
        "add esp, 4",
        "iretd",
        options(noreturn),
        );
    }
}