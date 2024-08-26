// Project Name:  MinmusOS
// File Name:     color.rs
// File Function: The implementation of the command echo
// Author:        Jishen Lin
// License:       MIT License

pub fn echo(b: &[char]) {
    let mut last_was_space = true;
    for &ch in b.iter().skip(5) {
        if ch == '\0' {
            break;
        }
        if ch == ' ' {
            if !last_was_space {
                lib::print!(" ");
                last_was_space = true;
            }
        } else {
            lib::print!("{}", ch);
            last_was_space = false;
        }
    }
    lib::println!();
}