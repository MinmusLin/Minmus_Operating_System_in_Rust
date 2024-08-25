// Project Name:  MinmusOS
// File Name:     main.rs
// File Function: The entry of application hanoi
// Author:        Jishen Lin
// License:       MIT License

#![no_std]
#![no_main]

use core::panic::PanicInfo;
use lib;
use lib::{print, println};

const STACK_SIZE: usize = 10;
struct HanoiTowers {
    a: [i32; STACK_SIZE],
    b: [i32; STACK_SIZE],
    c: [i32; STACK_SIZE],
    a_top: usize,
    b_top: usize,
    c_top: usize,
    count: usize,
}

fn main() {
    let mut towers = HanoiTowers {
        a: [0; STACK_SIZE],
        b: [0; STACK_SIZE],
        c: [0; STACK_SIZE],
        a_top: 0,
        b_top: STACK_SIZE,
        c_top: STACK_SIZE,
        count: 0,
    };
    for i in 0..STACK_SIZE {
        towers.a[i] = (i + 1) as i32;
    }
    println!("Hanoi Tower with {} disks:", STACK_SIZE);
    print!("#{:>4}     ", towers.count);
    print_stacks(&towers);
    move_disks(STACK_SIZE as i32, 'A', 'C', 'B', &mut towers);
}

fn move_disks(n: i32, from: char, to: char, aux: char, towers: &mut HanoiTowers) {
    if n == 1 {
        move_one_disk(from, to, towers);
    } else {
        move_disks(n - 1, from, aux, to, towers);
        move_one_disk(from, to, towers);
        move_disks(n - 1, aux, to, from, towers);
    }
}

fn move_one_disk(from: char, to: char, towers: &mut HanoiTowers) {
    towers.count += 1;
    print!("#{:>4} {}->{}", towers.count, from, to);
    transfer_disk(from, to, towers);
    print_stacks(towers);
}

fn transfer_disk(from: char, to: char, towers: &mut HanoiTowers) {
    let (from_stack, to_stack, from_top, to_top) = match (from, to) {
        ('A', 'B') => (&mut towers.a, &mut towers.b, &mut towers.a_top, &mut towers.b_top),
        ('A', 'C') => (&mut towers.a, &mut towers.c, &mut towers.a_top, &mut towers.c_top),
        ('B', 'A') => (&mut towers.b, &mut towers.a, &mut towers.b_top, &mut towers.a_top),
        ('B', 'C') => (&mut towers.b, &mut towers.c, &mut towers.b_top, &mut towers.c_top),
        ('C', 'A') => (&mut towers.c, &mut towers.a, &mut towers.c_top, &mut towers.a_top),
        ('C', 'B') => (&mut towers.c, &mut towers.b, &mut towers.c_top, &mut towers.b_top),
        _ => return,
    };
    if *from_top < STACK_SIZE {
        let disk: i32 = from_stack[*from_top];
        from_stack[*from_top] = 0;
        *from_top += 1;
        if *to_top > 0 {
            *to_top -= 1;
            to_stack[*to_top] = disk;
        }
    }
}

fn print_stacks(towers: &HanoiTowers) {
    print!(" A ");
    for a in (0..STACK_SIZE).rev() {
        if a >= towers.a_top {
            print!("{:>2}", towers.a[a]);
        } else {
            print!("  ");
        }
    }
    print!(" B ");
    for b in (0..STACK_SIZE).rev() {
        if b >= towers.b_top {
            print!("{:>2}", towers.b[b]);
        } else {
            print!("  ");
        }
    }
    print!(" C ");
    for c in (0..STACK_SIZE).rev() {
        if c >= towers.c_top {
            print!("{:>2}", towers.c[c]);
        } else {
            print!("  ");
        }
    }
    println!();
}

#[no_mangle]
#[link_section = ".start"]
pub extern "C" fn _start() {
    main();
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}