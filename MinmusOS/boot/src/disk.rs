// Project Name:  MinmusOS
// File Name:     disk.rs
// File Function: Disk reader
// Author:        Jishen Lin
// License:       MIT License

const SECTOR_SIZE: u16 = 512;

#[repr(C, packed)]
struct DiskAddressPacket {
    size: u8,
    zero: u8,
    sectors: u16,
    offset: u16,
    segment: u16,
    lba: u64,
}

pub struct DiskReader {
    lba: u64,
    target: u16,
}

impl DiskReader {
    pub fn new(lba: u64, target: u16) -> Self {
        Self {
            lba,
            target,
        }
    }

    pub fn read_sector(&self) {
        let dap = DiskAddressPacket {
            size: size_of::<DiskAddressPacket>() as u8,
            zero: 0,
            sectors: 1,
            offset: self.target,
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

    pub fn read_sectors(&mut self, sectors: u16) {
        let mut sectors_left = sectors;
        while sectors_left > 0 {
            self.read_sector();
            self.target += SECTOR_SIZE;
            self.lba += 1;
            sectors_left -= 1;
        }
    }
}