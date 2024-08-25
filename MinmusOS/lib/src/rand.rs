// Project Name:  MinmusOS
// File Name:     rand.rs
// File Function: Rand utils
// Author:        Jishen Lin
// License:       MIT License

pub struct LCG {
    state: u32,
    a: u32,
    c: u32,
    m: u32,
}

impl LCG {
    pub fn new(seed: u32) -> Self {
        LCG {
            state: seed,
            a: 1664525,
            c: 1013904223,
            m: 2_u32.pow(32),
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state = (self.a.wrapping_mul(self.state).wrapping_add(self.c)) % self.m;
        self.state
    }
}

pub struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    pub fn new(seed: u32) -> Self {
        Xorshift32 {
            state: seed,
        }
    }

    pub fn next(&mut self) -> u32 {
        let mut x: u32 = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        x
    }
}

pub struct MiddleSquare {
    state: u32,
}

impl MiddleSquare {
    pub fn new(seed: u32) -> Self {
        MiddleSquare {
            state: seed,
        }
    }

    pub fn next(&mut self) -> u32 {
        let squared: u32 = self.state.wrapping_mul(self.state);
        let middle: u32 = (squared >> 8) & 0xFFFF;
        self.state = middle;
        middle
    }
}

pub struct FibonacciLCG {
    state1: u32,
    state2: u32,
    m: u32,
}

impl FibonacciLCG {
    pub fn new(seed1: u32, seed2: u32) -> Self {
        FibonacciLCG {
            state1: seed1,
            state2: seed2,
            m: 2_u32.pow(32),
        }
    }

    pub fn next(&mut self) -> u32 {
        let next_value: u32 = (self.state1.wrapping_add(self.state2)) % self.m;
        self.state1 = self.state2;
        self.state2 = next_value;
        next_value
    }
}

pub struct MersenneTwister {
    state: [u32; 624],
    index: usize,
}

impl MersenneTwister {
    pub fn new(seed: u32) -> Self {
        let mut mt = MersenneTwister {
            state: [0; 624],
            index: 624,
        };
        mt.state[0] = seed;
        for i in 1..624 {
            mt.state[i] = 0x6c078965_u32.wrapping_mul(mt.state[i - 1] ^ (mt.state[i - 1] >> 30)).wrapping_add(i as u32);
        }
        mt
    }

    pub fn next(&mut self) -> u32 {
        if self.index >= 624 {
            self.twist();
        }
        let mut y: u32 = self.state[self.index];
        y ^= y >> 11;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;
        self.index += 1;
        y
    }

    fn twist(&mut self) {
        for i in 0..624 {
            let y: u32 = (self.state[i] & 0x80000000) + (self.state[(i + 1) % 624] & 0x7fffffff);
            self.state[i] = self.state[(i + 397) % 624] ^ (y >> 1);
            if y % 2 != 0 {
                self.state[i] ^= 0x9908b0df;
            }
        }
        self.index = 0;
    }
}

pub struct TimeSeed {
    state: u32,
}

impl TimeSeed {
    pub fn new(start_time: u64) -> Self {
        TimeSeed {
            state: (start_time & 0xFFFFFFFF) as u32,
        }
    }

    pub fn next(&mut self) -> u32 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 17;
        self.state ^= self.state << 5;
        self.state
    }
}

pub struct LFSR {
    state: u32,
}

impl LFSR {
    pub fn new(seed: u32) -> Self {
        LFSR {
            state: seed,
        }
    }

    pub fn next(&mut self) -> u32 {
        let bit: u32 = ((self.state >> 0) ^ (self.state >> 2) ^ (self.state >> 3) ^ (self.state >> 5)) & 1;
        self.state = (self.state >> 1) | (bit << 31);
        self.state
    }
}

pub struct CombinedGenerator {
    gen1: LCG,
    gen2: Xorshift32,
}

impl CombinedGenerator {
    pub fn new(seed1: u32, seed2: u32) -> Self {
        CombinedGenerator {
            gen1: LCG::new(seed1),
            gen2: Xorshift32::new(seed2),
        }
    }

    pub fn next(&mut self) -> u32 {
        let r1: u32 = self.gen1.next();
        let r2: u32 = self.gen2.next();
        r1 ^ r2
    }
}