// Project Name:  MinmusOS
// File Name:     calendar.rs
// File Function: Calendar
// Author:        Jishen Lin
// License:       MIT License

use crate::syscalls::print::{COLOR_BLACK, COLOR_LIGHT_YELLOW, PRINTER};
use crate::timer::time::Time;

pub unsafe fn cal() {
    let time: Time = Time::init();
    let year: u16 = time.get_year();
    let month: u8 = time.get_month();
    let day: u8 = time.get_day();

    let month_title: &str = match month {
        1 => "January",
        2 => "February",
        3 => "March",
        4 => "April",
        5 => "May",
        6 => "June",
        7 => "July",
        8 => "August",
        9 => "September",
        10 => "October",
        11 => "November",
        12 => "December",
        _ => "Invalid Month",
    };

    lib::println!("[{} {}]", month_title, year);
    lib::println!("Mon Tue Wed Thu Fri Sat Sun");

    let first_day_of_month: usize = {
        let (y, m): (i32, u8) = if month < 3 { (year as i32 - 1, month + 12) } else { (year as i32, month) };
        let k: i32 = y % 100;
        let j: i32 = y / 100;
        let h: i32 = (1 + (13 * (m as i32 + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
        ((h + 5) % 7) as usize
    };

    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => { if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) { 29 } else { 28 } }
        _ => 30,
    };

    for _ in 0..first_day_of_month {
        lib::print!("    ");
    }

    for date in 1..=days_in_month {
        if date == day {
            PRINTER.set_colors(COLOR_LIGHT_YELLOW, COLOR_BLACK);
            lib::print!("{:>3} ", date);
            PRINTER.reset_colors();
        } else {
            lib::print!("{:>3} ", date);
        }
        if (first_day_of_month as u8 + date) % 7 == 0 {
            if date != days_in_month {
                lib::println!();
            }
        }
    }

    lib::println!();
}