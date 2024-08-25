// Project Name:  MinmusOS
// File Name:     shell.rs
// File Function: Shell
// Author:        Jishen Lin
// License:       MIT License

use crate::filesystem::fat::{Entry, FatDriver, FAT};
use crate::memory::paging::PAGING;
use crate::memory::paging::TABLES;
use crate::multitasking::task::TASK_MANAGER;
use crate::syscalls::print::{COLOR_BLACK, COLOR_LIGHT_CYAN, COLOR_LIGHT_GREEN, COLOR_LIGHT_MAGENTA, COLOR_LIGHT_RED, COLOR_LIGHT_WHITE, PRINTER};
use crate::timer::time::Time;

const APP_TARGET: u32 = 0x00A00000;
const APP_SIZE: u32 = 0x00010000;
const APP_SIGNATURE: u32 = 0xB16B00B5;
const HELP: &'static str = "Available commands:
cal            - Shows current month's calendar
cat <filename> - Shows content of a file
clear          - Clears terminal screen
color          - Shows VGA text mode color
date           - Shows current datetime
echo <text>    - Outputs text
exit           - Exits current session
help           - Shows available commands
hostname       - Shows hostname
kill <pid>     - Terminates specified process
ls             - Lists root directory entries
ps             - Lists running tasks
pwd            - Shows current directory
reboot         - Reboot system
run <appname>  - Runs an application
shutdown       - Shutdowns system
ticks          - Shows current CPU ticks
timestamp      - Shows current timestamp
uname          - Shows system information
whoami         - Shows current user";

pub static mut SHELL: Shell = Shell {
    buffer: [0 as char; 256],
    arg: [0 as char; 11],
    cursor: 0,
};

pub struct Shell {
    buffer: [char; 256],
    arg: [char; 11],
    cursor: usize,
}

impl Shell {
    pub fn init(&mut self) {
        self.buffer = ['\0'; 256];
        self.cursor = 0;
        unsafe {
            PRINTER.set_colors(COLOR_LIGHT_GREEN, COLOR_BLACK);
            lib::print!("root@MinmusOS");
            PRINTER.set_colors(COLOR_LIGHT_WHITE, COLOR_BLACK);
            lib::print!(":");
            PRINTER.set_colors(COLOR_LIGHT_CYAN, COLOR_BLACK);
            lib::print!("/");
            PRINTER.set_colors(COLOR_LIGHT_WHITE, COLOR_BLACK);
            lib::print!("$ ");
            PRINTER.reset_colors();
        }
    }

    pub fn add(&mut self, c: char) {
        self.buffer[self.cursor] = c;
        self.cursor += 1;
        lib::print!("{}", c);
    }

    pub fn backspace(&mut self) {
        if self.cursor > 0 {
            unsafe {
                PRINTER.delete();
            }
            self.cursor -= 1;
            self.buffer[self.cursor] = 0 as char;
        }
    }

    pub fn enter(&mut self) {
        unsafe {
            PRINTER.new_line();
        }
        self.interpret();
        self.init();
    }

    #[allow(unused_unsafe)]
    fn interpret(&mut self) {
        match self.buffer {
            _b if self.is_command("cal") => unsafe {
                crate::shell::cal::cal();
            },
            b if self.is_command("cat") => unsafe {
                self.cat(&b);
            },
            _b if self.is_command("clear") => unsafe {
                PRINTER.clear();
            },
            _b if self.is_command("color") => unsafe {
                crate::shell::color::color();
            },
            _b if self.is_command("date") => unsafe {
                let time = Time::init();
                lib::println!("{:04}-{:02}-{:02} {:02}:{:02}:{:02} UTC", time.get_year(), time.get_month(), time.get_day(), time.get_hour(), time.get_minute(), time.get_second());
            },
            b if self.is_command("echo") => unsafe {
                crate::shell::echo::echo(&b);
            },
            _b if self.is_command("exit") => unsafe {
                core::arch::asm!("mov dx, 0x604", "mov ax, 0x2000", "out dx, ax", options(nostack, nomem));
            },
            _b if self.is_command("help") => unsafe {
                lib::println!("{}", HELP);
            },
            _b if self.is_command("hostname") => unsafe {
                lib::println!("MinmusOS");
            },
            b if self.is_command("kill") => unsafe {
                crate::shell::kill::kill(&b);
            },
            _b if self.is_command("ls") => unsafe {
                FAT.acquire().list_entries();
                FAT.free();
            },
            _b if self.is_command("ps") => unsafe {
                TASK_MANAGER.list_tasks();
            },
            _b if self.is_command("pwd") => unsafe {
                lib::println!("/");
            },
            _b if self.is_command("reboot") => unsafe {
                core::arch::asm!("mov al, 0xFE", "out 0x64, al", options(nostack, nomem));
            },
            b if self.is_command("run") => unsafe {
                self.run(&b);
            },
            _b if self.is_command("shutdown") => unsafe {
                core::arch::asm!("mov dx, 0x604", "mov ax, 0x2000", "out dx, ax", options(nostack, nomem));
            },
            _b if self.is_command("ticks") => unsafe {
                lib::println!("{}", Time::init().get_ticks());
            },
            _b if self.is_command("timestamp") => unsafe {
                lib::println!("{}", Time::init().get_timestamp());
            },
            _b if self.is_command("uname") => unsafe {
                lib::println!("MinmusOS v1.0 IA-32 x86");
            },
            _b if self.is_command("whoami") => unsafe {
                lib::println!("root");
            },
            b if b[0] == '\0' || b[0] == '\n' => {}
            _ => {
                unsafe {
                    PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
                }
                lib::println!("Command not found!");
                unsafe {
                    PRINTER.reset_colors();
                }
            }
        }
    }

    unsafe fn cat(&mut self, b: &[char]) {
        let start: usize = b.iter().skip(4).position(|&c| c != ' ' && c != '\0').unwrap_or(b.len());
        if start + 4 >= b.len() {
            PRINTER.set_colors(COLOR_LIGHT_MAGENTA, COLOR_BLACK);
            lib::println!("Usage: cat <filename>");
            PRINTER.reset_colors();
            return;
        }
        let end: usize = b.iter().skip(start + 4).position(|&c| c == ' ' || c == '\0').map_or(b.len(), |p| p + start + 4);
        if end <= start + 4 {
            PRINTER.set_colors(COLOR_LIGHT_MAGENTA, COLOR_BLACK);
            lib::println!("Usage: cat <filename>");
            PRINTER.reset_colors();
            return;
        }
        let mut j: usize = 0;
        for i in start + 4..end {
            self.arg[j] = b[i];
            j += 1;
        }
        for i in j..self.arg.len() {
            self.arg[i] = '\0';
        }
        let fat: &FatDriver = FAT.acquire();
        let entry: &Entry = fat.search_file(&self.arg);
        if entry.name[0] != 0 {
            fat.read_file_to_buffer(entry);
            for c in fat.buffer {
                if c != 0 {
                    lib::print!("{}", c as char);
                }
            }
            lib::println!();
        } else {
            PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
            lib::println!("File not found!");
            PRINTER.reset_colors();
        }
        FAT.free();
    }

    unsafe fn run(&mut self, b: &[char]) {
        let start: usize = b.iter().skip(4).position(|&c| c != ' ' && c != '\0').unwrap_or(b.len());
        if start + 4 >= b.len() {
            unsafe {
                PRINTER.set_colors(COLOR_LIGHT_MAGENTA, COLOR_BLACK);
            }
            lib::println!("Usage: run <appname>");
            unsafe {
                PRINTER.reset_colors();
            }
            return;
        }
        let end: usize = b.iter().skip(start + 4).position(|&c| c == ' ' || c == '\0').map_or(b.len(), |p| p + start + 4);
        if end <= start + 4 {
            unsafe {
                PRINTER.set_colors(COLOR_LIGHT_MAGENTA, COLOR_BLACK);
            }
            lib::println!("Usage: run <appname>");
            unsafe {
                PRINTER.reset_colors();
            }
            return;
        }
        let mut j: usize = 0;
        for i in start + 4..end {
            self.arg[j] = b[i];
            j += 1;
        }
        for i in j..self.arg.len() {
            self.arg[i] = '\0';
        }
        let fat: &FatDriver = FAT.acquire();
        let entry: &Entry = fat.search_file(&self.arg);
        if entry.name[0] != 0 {
            let target: u32 = APP_TARGET + (TASK_MANAGER.get_free_slot() as u32 * APP_SIZE);
            TABLES[8].set(target);
            PAGING.set_table(8, &TABLES[8]);
            fat.read_file_to_target(&entry, target as *mut u32);
            if *(target as *mut u32) == APP_SIGNATURE {
                TASK_MANAGER.add_task(target + 4);
            } else {
                unsafe {
                    PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
                }
                lib::println!("This file is not a valid executable!");
                unsafe {
                    PRINTER.reset_colors();
                }
            }
        } else {
            PRINTER.set_colors(COLOR_LIGHT_RED, COLOR_BLACK);
            lib::println!("Application not found!");
            PRINTER.reset_colors();
        }
        FAT.free();
    }

    fn is_command(&self, command: &str) -> bool {
        let command_len: usize = command.len();
        for (i, command_char) in command.chars().enumerate() {
            let buffer_char: char = self.buffer[i].to_ascii_lowercase();
            if buffer_char == '\0' || buffer_char != command_char.to_ascii_lowercase() {
                return false;
            }
        }
        self.buffer[command_len].to_ascii_lowercase() == ' ' || self.buffer[command_len] == '\0'
    }
}