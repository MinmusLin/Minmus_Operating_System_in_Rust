// Project Name:  MinmusOS
// File Name:     string.rs
// File Function: String utils
// Author:        Jishen Lin
// License:       MIT License

pub fn strlen(s: &[char]) -> usize {
    let mut len: usize = 0;
    while len < s.len() && s[len] != '\0' {
        len += 1;
    }
    len
}

pub fn strcat<'a>(s1: &'a mut [char], s2: &[char]) -> &'a mut [char] {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < s1.len() && s1[i] != '\0' {
        i += 1;
    }
    while j < s2.len() && i < s1.len() {
        s1[i] = s2[j];
        i += 1;
        j += 1;
    }
    s1
}

pub fn strncat<'a>(s1: &'a mut [char], s2: &[char], len: usize) -> &'a mut [char] {
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < s1.len() && s1[i] != '\0' {
        i += 1;
    }
    while j < len && j < s2.len() && i < s1.len() {
        s1[i] = s2[j];
        i += 1;
        j += 1;
    }
    s1
}

pub fn strcpy<'a>(s1: &'a mut [char], s2: &[char]) -> &'a mut [char] {
    let mut i: usize = 0;
    while i < s2.len() && i < s1.len() {
        s1[i] = s2[i];
        i += 1;
    }
    if i < s1.len() {
        s1[i] = '\0';
    }
    s1
}

pub fn strncpy<'a>(s1: &'a mut [char], s2: &[char], len: usize) -> &'a mut [char] {
    let mut i: usize = 0;
    while i < len && i < s2.len() && i < s1.len() {
        s1[i] = s2[i];
        i += 1;
    }
    while i < len && i < s1.len() {
        s1[i] = '\0';
        i += 1;
    }
    s1
}

pub fn strcmp(s1: &[char], s2: &[char]) -> i32 {
    let mut i: usize = 0;
    while i < s1.len() && i < s2.len() {
        if s1[i] != s2[i] {
            return s1[i] as i32 - s2[i] as i32;
        }
        i += 1;
    }
    s1.len() as i32 - s2.len() as i32
}

pub fn strcasecmp(s1: &[char], s2: &[char]) -> i32 {
    let mut i: usize = 0;
    while i < s1.len() && i < s2.len() {
        let c1: char = if s1[i].is_uppercase() { s1[i].to_ascii_lowercase() } else { s1[i] };
        let c2: char = if s2[i].is_uppercase() { s2[i].to_ascii_lowercase() } else { s2[i] };
        if c1 != c2 {
            return c1 as i32 - c2 as i32;
        }
        i += 1;
    }
    s1.len() as i32 - s2.len() as i32
}

pub fn strncmp(s1: &[char], s2: &[char], len: usize) -> i32 {
    let mut i: usize = 0;
    while i < len && i < s1.len() && i < s2.len() {
        if s1[i] != s2[i] {
            return s1[i] as i32 - s2[i] as i32;
        }
        i += 1;
    }
    if i < len {
        s1.len() as i32 - s2.len() as i32
    } else {
        0
    }
}

pub fn strcasencmp(s1: &[char], s2: &[char], len: usize) -> i32 {
    let mut i: usize = 0;
    while i < len && i < s1.len() && i < s2.len() {
        let c1: char = if s1[i].is_uppercase() { s1[i].to_ascii_lowercase() } else { s1[i] };
        let c2: char = if s2[i].is_uppercase() { s2[i].to_ascii_lowercase() } else { s2[i] };
        if c1 != c2 {
            return c1 as i32 - c2 as i32;
        }
        i += 1;
    }
    if i < len {
        s1.len() as i32 - s2.len() as i32
    } else {
        0
    }
}

pub fn strupr(s: &mut [char]) -> &mut [char] {
    let mut i: usize = 0;
    while i < s.len() && s[i] != '\0' {
        if s[i].is_lowercase() {
            s[i] = s[i].to_ascii_uppercase();
        }
        i += 1;
    }
    s
}

pub fn strlwr(s: &mut [char]) -> &mut [char] {
    let mut i: usize = 0;
    while i < s.len() && s[i] != '\0' {
        if s[i].is_uppercase() {
            s[i] = s[i].to_ascii_lowercase();
        }
        i += 1;
    }
    s
}

pub fn strchr(s: &[char], ch: char) -> i32 {
    let mut i: usize = 0;
    while i < s.len() {
        if s[i] == ch {
            return i as i32;
        }
        i += 1;
    }
    -1
}

pub fn strstr(s: &[char], substr: &[char]) -> i32 {
    if substr.is_empty() {
        return 0;
    }
    let mut i: usize = 0;
    while i <= s.len() - substr.len() {
        let mut j: usize = 0;
        while j < substr.len() && s[i + j] == substr[j] {
            j += 1;
        }
        if j == substr.len() {
            return i as i32;
        }
        i += 1;
    }
    -1
}

pub fn strrchr(s: &[char], ch: char) -> i32 {
    let mut i: i32 = s.len() as i32 - 1;
    while i >= 0 {
        if s[i as usize] == ch {
            return i;
        }
        i -= 1;
    }
    -1
}

pub fn strrstr(s: &[char], substr: &[char]) -> i32 {
    if substr.is_empty() {
        return s.len() as i32;
    }
    let mut i: usize = s.len() - substr.len();
    loop {
        let mut j: usize = 0;
        while j < substr.len() && s[i + j] == substr[j] {
            j += 1;
        }
        if j == substr.len() {
            return i as i32;
        }
        if i == 0 {
            break;
        }
        i -= 1;
    }
    -1
}

pub fn strrev(s: &mut [char]) -> &mut [char] {
    let mut i: usize = 0;
    let mut j: usize = s.len() - 1;
    while i < j {
        let temp: char = s[i];
        s[i] = s[j];
        s[j] = temp;
        i += 1;
        j -= 1;
    }
    s
}