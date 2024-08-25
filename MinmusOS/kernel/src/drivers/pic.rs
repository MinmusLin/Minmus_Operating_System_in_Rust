// Project Name:  MinmusOS
// File Name:     pic.rs
// File Function: Programmable interrupt controllers driver
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;

const MASTER_PIC_COMMAND_PORT: u8 = 0x20;
const MASTER_PIC_DATA_PORT: u8 = 0x21;
const SLAVE_PIC_COMMAND_PORT: u8 = 0xA0;
const SLAVE_PIC_DATA_PORT: u8 = 0xA1;
const COMMAND_INIT: u8 = 0x11;
const COMMAND_EOF: u8 = 0x20;
const MODE: u8 = 0x01;
const OFFSET: u8 = 32;
const IRQ_COUNT: u8 = 8;

pub static PICS: Pics = Pics {
    master: Pic {
        offset: OFFSET,
        command_port: MASTER_PIC_COMMAND_PORT,
        data_port: MASTER_PIC_DATA_PORT,
    },
    slave: Pic {
        offset: OFFSET + IRQ_COUNT,
        command_port: SLAVE_PIC_COMMAND_PORT,
        data_port: SLAVE_PIC_DATA_PORT,
    },
};

struct Pic {
    offset: u8,
    command_port: u8,
    data_port: u8,
}

impl Pic {
    pub fn read_data(&self) -> u8 {
        let data: u8;
        unsafe {
            asm!("in al, dx", out("al") data, in("dx") self.data_port as u16);
        }
        data
    }

    pub fn write_data(&self, data: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.data_port as u16, in("al") data);
        }
    }

    pub fn send_command(&self, command: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.command_port as u16, in("al") command);
        }
    }

    pub fn end_interrupt(&self) {
        unsafe {
            asm!("out dx, al", in("dx") self.command_port as u16, in("al") COMMAND_EOF);
        }
    }

    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.offset <= interrupt && interrupt < self.offset + IRQ_COUNT
    }
}

pub struct Pics {
    master: Pic,
    slave: Pic,
}

impl Pics {
    pub fn init(&self) {
        let mask1 = self.master.read_data();
        let mask2 = self.slave.read_data();
        self.master.send_command(COMMAND_INIT);
        wait();
        self.slave.send_command(COMMAND_INIT);
        wait();
        self.master.write_data(self.master.offset);
        wait();
        self.slave.write_data(self.slave.offset);
        wait();
        self.master.write_data(4);
        wait();
        self.slave.write_data(2);
        wait();
        self.master.write_data(MODE);
        wait();
        self.slave.write_data(MODE);
        wait();
        self.master.write_data(mask1);
        self.slave.write_data(mask2);
    }

    pub fn handles_interrupt(&self, interrupt: u8) -> bool {
        self.master.handles_interrupt(interrupt) || self.slave.handles_interrupt(interrupt)
    }

    pub fn end_interrupt(&self, interrupt: u8) {
        if self.handles_interrupt(interrupt) {
            if self.slave.handles_interrupt(interrupt) {
                self.slave.end_interrupt();
            }
            self.master.end_interrupt();
        }
    }
}

pub fn wait() {
    unsafe {
        asm!("out dx, al", in("dx") 0x80u16, in("al") 0u8);
    }
}