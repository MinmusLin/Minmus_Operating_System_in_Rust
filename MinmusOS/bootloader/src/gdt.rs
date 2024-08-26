// Project Name:  MinmusOS
// File Name:     gdt.rs
// File Function: Global descriptor table
// Author:        Jishen Lin
// License:       MIT License

const GDT_ENTRIES: usize = 3;

pub static GDT: GlobalDescriptorTable = {
    let limit: u64 = {
        let limit_low: u64 = 0xFFFF << 0;
        let limit_high: u64 = 0xF << 48;
        limit_low | limit_high
    };

    let base: u64 = {
        let base_low: u64 = 0x0000 << 16;
        let base_high: u64 = 0x00 << 56;
        base_low | base_high
    };

    let access: u64 = {
        let p: u64 = 0b1 << 47;
        let dpl: u64 = 0b00 << 46;
        let s: u64 = 0b1 << 44;
        let e: u64 = 0b0 << 43;
        let dc: u64 = 0b0 << 42;
        let rw: u64 = 0b1 << 41;
        let a: u64 = 0b0 << 40;
        p | dpl | s | e | dc | rw | a
    };

    let flags: u64 = {
        let g: u64 = 0b1 << 55;
        let db: u64 = 0b1 << 54;
        let l: u64 = 0b0 << 53;
        let r: u64 = 0b0 << 52;
        g | db | l | r
    };

    let executable: u64 = 0b1 << 43;

    let zero = GdtEntry {
        entry: 0
    };

    let code = GdtEntry {
        entry: limit | base | access | flags | executable
    };

    let data = GdtEntry {
        entry: limit | base | access | flags
    };

    GlobalDescriptorTable {
        entries: [zero, code, data]
    }
};

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct GdtEntry {
    entry: u64,
}

#[repr(C, packed)]
pub struct GlobalDescriptorTable {
    entries: [GdtEntry; GDT_ENTRIES],
}

#[repr(C, packed)]
pub struct GdtDescriptor {
    size: u16,
    offset: *const GlobalDescriptorTable,
}

impl GlobalDescriptorTable {
    pub fn load(&self) {
        let descriptor = GdtDescriptor {
            size: (GDT_ENTRIES * size_of::<GdtEntry>() - 1) as u16,
            offset: self,
        };
        unsafe {
            core::arch::asm!("lgdt [{0:e}]", in(reg) &descriptor);
        }
    }
}