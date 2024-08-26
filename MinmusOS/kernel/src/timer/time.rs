// Project Name:  MinmusOS
// File Name:     timer.rs
// File Function: Timer
// Author:        Jishen Lin
// License:       MIT License

use core::arch::asm;

#[derive(Copy, Clone, Debug)]
pub struct Time {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    timestamp: u64,
    ticks: u64,
}

impl Time {
    pub fn init() -> Self {
        let current_datetime: (u16, u8, u8, u8, u8, u8) = Self::get_current_datetime();
        let timestamp: u64 = Self::get_unix_timestamp(current_datetime);
        let ticks: u64 = Self::get_cpu_ticks();
        Time {
            year: current_datetime.0,
            month: current_datetime.1,
            day: current_datetime.2,
            hour: current_datetime.3,
            minute: current_datetime.4,
            second: current_datetime.5,
            timestamp,
            ticks,
        }
    }

    pub fn get_year(&self) -> u16 {
        self.year
    }

    pub fn get_month(&self) -> u8 {
        self.month
    }

    pub fn get_day(&self) -> u8 {
        self.day
    }

    pub fn get_hour(&self) -> u8 {
        self.hour
    }

    pub fn get_minute(&self) -> u8 {
        self.minute
    }

    pub fn get_second(&self) -> u8 {
        self.second
    }

    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn get_ticks(&self) -> u64 {
        self.ticks
    }

    fn get_current_datetime() -> (u16, u8, u8, u8, u8, u8) {
        let mut second: u8;
        let mut minute: u8;
        let mut hour: u8;
        let mut day: u8;
        let mut month: u8;
        let mut year: u8;
        unsafe {
            asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") 0x00u8,
            lateout("al") second,
            );
            asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") 0x02u8,
            lateout("al") minute,
            );
            asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") 0x04u8,
            lateout("al") hour,
            );
            asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") 0x07u8,
            lateout("al") day,
            );
            asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") 0x08u8,
            lateout("al") month,
            );
            asm!(
            "out 0x70, al",
            "in al, 0x71",
            in("al") 0x09u8,
            lateout("al") year,
            );
        }
        second = Self::bcd_to_binary(second);
        minute = Self::bcd_to_binary(minute);
        hour = Self::bcd_to_binary(hour);
        day = Self::bcd_to_binary(day);
        month = Self::bcd_to_binary(month);
        year = Self::bcd_to_binary(year);
        (2000 + year as u16, month, day, hour, minute, second)
    }

    fn get_unix_timestamp(datetime: (u16, u8, u8, u8, u8, u8)) -> u64 {
        let (year, month, day, hour, minute, second) = datetime;
        let mut days_since_epoch: u64 = 0;
        for y in 1970..year {
            days_since_epoch += if Self::is_leap_year(y) { 366 } else { 365 };
        }
        let month_days: [i32; 13] = if Self::is_leap_year(year) {
            [0, 31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        } else {
            [0, 31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]
        };
        for m in 1..month {
            days_since_epoch += month_days[m as usize] as u64;
        }
        days_since_epoch += day as u64 - 1;
        let hours_since_epoch: u64 = days_since_epoch * 24 + hour as u64;
        let minutes_since_epoch: u64 = hours_since_epoch * 60 + minute as u64;
        let seconds_since_epoch: u64 = minutes_since_epoch * 60 + second as u64;
        seconds_since_epoch
    }

    fn get_cpu_ticks() -> u64 {
        let mut low: u32;
        let mut high: u32;
        unsafe {
            asm!(
            "rdtsc",
            out("eax") low,
            out("edx") high,
            options(nostack, nomem),
            );
        }
        ((high as u64) << 32) | (low as u64)
    }

    fn bcd_to_binary(bcd: u8) -> u8 {
        (bcd & 0x0F) + ((bcd >> 4) * 10)
    }

    fn is_leap_year(year: u16) -> bool {
        (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
    }
}