// Project Name:  MinmusOS
// File Name:     disk.rs
// File Function: Disk reader
// Author:        Jishen Lin
// License:       MIT License

const SECTOR_SIZE: u64 = 512;

pub static mut DISK: Disk = Disk {
    lba: 0,
    buffer: 0,
};

#[repr(C, packed)]
struct DiskAddressPacket {
    size: u8,
    zero: u8,
    sectors: u16,
    offset: u16,
    segment: u16,
    lba: u64,
}

pub struct Disk {
    lba: u64,
    buffer: u16,
}

impl Disk {
    pub fn init(&mut self, lba: u64, buffer: u16) {
        self.lba = lba;
        self.buffer = buffer;
    }

    pub fn read_sector(&self) {
        let dap = DiskAddressPacket {
            size: size_of::<DiskAddressPacket>() as u8,
            zero: 0,
            sectors: 1,
            offset: self.buffer,
            segment: 0x0000,
            lba: self.lba,
        };

        let dap_address = &dap as *const DiskAddressPacket;

        unsafe {
            core::arch::asm!(
            "mov {1:x}, si",
            "mov si, {0:x}",
            "int 0x13",
            "jc fail",
            "mov si, {1:x}",
            in(reg) dap_address as u16,
            out(reg) _,
            in("ax") 0x4200u16,
            in("dx") 0x0080u16,
            );
        }
    }

    pub fn read_sectors(&mut self, sectors: u16, target: u32) {
        let mut sectors_left = sectors;
        let mut current_target = target;
        while sectors_left > 0 {
            self.read_sector();
            let mut byte_address = self.buffer;
            for _byte_index in 0..SECTOR_SIZE {
                unsafe {
                    let mut byte: u8;
                    core::arch::asm!("mov {0}, [{1:e}]", out(reg_byte) byte, in(reg) byte_address);
                    core::arch::asm!("mov [{0:e}], {1}", in(reg) current_target, in(reg_byte) byte);
                }
                current_target += 1;
                byte_address += 1;
            }
            self.lba += 1;
            sectors_left -= 1;
        }
    }
}