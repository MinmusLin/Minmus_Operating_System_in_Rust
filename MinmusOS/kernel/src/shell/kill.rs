// Project Name:  MinmusOS
// File Name:     color.rs
// File Function: The implementation of the command kill
// Author:        Jishen Lin
// License:       MIT License

use crate::multitasking::task::{MAX_TASKS, TASK_MANAGER};
use crate::syscalls::print::{COLOR_BLACK, COLOR_LIGHT_MAGENTA, COLOR_LIGHT_RED, PRINTER};

pub fn kill(b: &[char]) {
    let start: usize = b.iter().skip(5).position(|&c| c != ' ' && c != '\0').unwrap_or(b.len());
    if start + 5 >= b.len() {
        unsafe {
            PRINTER.set_colors(COLOR_LIGHT_MAGENTA, COLOR_BLACK);
        }
        lib::println!("Usage: kill <pid>");
        unsafe {
            PRINTER.reset_colors();
        }
        return;
    }
    let end: usize = b.iter().skip(start + 5).position(|&c| c == ' ' || c == '\0').map_or(b.len(), |p| p + start + 5);
    if end <= start + 5 {
        unsafe {
            PRINTER.set_colors(COLOR_LIGHT_MAGENTA, COLOR_BLACK);
        }
        lib::println!("Usage: kill <pid>");
        unsafe {
            PRINTER.reset_colors();
        }
        return;
    }
    let mut task_id: usize = 0;
    let mut is_valid_id: bool = true;
    for i in start + 5..end {
        if b[i].is_digit(10) {
            task_id = task_id * 10 + (b[i] as u8 - b'0') as usize;
        } else {
            is_valid_id = false;
            break;
        }
    }
    if is_valid_id {
        if task_id > 0 && task_id < MAX_TASKS as usize {
            unsafe {
                if !TASK_MANAGER.tasks[task_id].running {
                    PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
                    lib::println!("Task with PID {} not found!", task_id);
                    PRINTER.reset_colors();
                    return;
                }
                TASK_MANAGER.remove_task(task_id);
                lib::println!("Task (PID {}) has been removed.", task_id);
            }
        } else {
            unsafe {
                PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
            }
            lib::println!("Please enter a valid PID (1-31).");
            unsafe {
                PRINTER.reset_colors();
            }
        }
    } else {
        unsafe {
            PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
        }
        lib::println!("Please enter a valid PID (1-31).");
        unsafe {
            PRINTER.reset_colors();
        }
    }
}