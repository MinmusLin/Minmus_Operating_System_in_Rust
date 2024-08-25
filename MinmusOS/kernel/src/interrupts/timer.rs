// Project Name:  MinmusOS
// File Name:     timer.rs
// File Function: Timer interrupt handler
// Author:        Jishen Lin
// License:       MIT License

use crate::drivers::pic::PICS;
use crate::multitasking::task::CPUState;
use crate::multitasking::task::TASK_MANAGER;
use crate::memory::paging::PAGING;
use crate::memory::paging::TABLES;

pub const TIMER_INT: u8 = 32;
const APP_TARGET: u32 = 0x00a0_0000;
const APP_SIZE: u32 = 0x0001_0000;

#[naked]
pub extern "C" fn timer() {
    unsafe {
        core::arch::asm!(
        "cli",
        "push ebp",
        "push edi",
        "push esi",
        "push edx",
        "push ecx",
        "push ebx",
        "push eax",
        "push esp",
        "call timer_handler",
        "mov esp, eax",
        "pop eax",
        "pop ebx",
        "pop ecx",
        "pop edx",
        "pop esi",
        "pop edi",
        "pop ebp",
        "sti",
        "iretd",
        options(noreturn),
        );
    }
}

#[no_mangle]
pub extern "C" fn timer_handler(esp: u32) -> u32 {
    unsafe {
        let new_esp: u32 = TASK_MANAGER.schedule(esp as *mut CPUState) as u32;
        let slot = TASK_MANAGER.get_current_slot();
        let target = APP_TARGET + (slot as u32 * APP_SIZE);
        TABLES[8].set(target);
        PAGING.set_table(8, &TABLES[8]);
        PICS.end_interrupt(TIMER_INT);
        new_esp
    }
}