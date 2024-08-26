// Project Name:  MinmusOS
// File Name:     math.rs
// File Function: Math utils
// Author:        Jishen Lin
// License:       MIT License

pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

pub fn sub(a: f64, b: f64) -> f64 {
    a - b
}

pub fn mul(a: f64, b: f64) -> f64 {
    a * b
}

pub fn div(a: f64, b: f64) -> f64 {
    a / b
}

pub fn max<T: PartialOrd>(a: T, b: T) -> T {
    if a > b {
        a
    } else {
        b
    }
}

pub fn min<T: PartialOrd>(a: T, b: T) -> T {
    if a < b {
        a
    } else {
        b
    }
}

pub fn ceil(x: f64) -> f64 {
    let floor: f64 = x as i64 as f64;
    if floor == x || x < 0.0 {
        floor
    } else {
        floor + 1.0
    }
}

pub fn floor(x: f64) -> f64 {
    let floor: f64 = x as i64 as f64;
    if floor <= x || x >= 0.0 {
        floor
    } else {
        floor - 1.0
    }
}

pub fn round(x: f64) -> f64 {
    if x < 0.0 {
        ceil(x - 0.5)
    } else {
        floor(x + 0.5)
    }
}

pub fn pow(mut base: f64, mut exp: i32) -> f64 {
    if exp == 0 {
        return 1.0;
    }
    let mut result: f64 = 1.0;
    let mut is_negative: bool = false;
    if exp < 0 {
        is_negative = true;
        exp = -exp;
    }
    while exp > 0 {
        if exp % 2 == 1 {
            result *= base;
        }
        base *= base;
        exp /= 2;
    }
    if is_negative {
        1.0 / result
    } else {
        result
    }
}

pub fn exp(x: f64) -> f64 {
    let mut term: f64 = 1.0;
    let mut sum: f64 = 1.0;
    for n in 1..100 {
        term *= x / n as f64;
        sum += term;
        if term < 1e-10 {
            break;
        }
    }
    sum
}

pub fn sqrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let mut z: f64 = x;
    let mut prev: f64 = 0.0;
    while fabs(z - prev) > 1e-6 {
        prev = z;
        z -= (z * z - x) / (2.0 * z);
    }
    z
}

pub fn cbrt(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    }
    let mut z: f64 = x;
    let mut prev: f64 = 0.0;
    while fabs(z - prev) > 1e-6 {
        prev = z;
        z -= (z * z * z - x) / (3.0 * z * z);
    }
    z
}

pub fn abs(value: i64) -> i64 {
    if value < 0 {
        -value
    } else {
        value
    }
}

pub fn fabs(value: f64) -> f64 {
    if value < 0.0 {
        -value
    } else {
        value
    }
}

pub fn sin(x: f64) -> f64 {
    let x_rad: f64 = x * core::f64::consts::PI / 180.0;
    let x3: f64 = x_rad * x_rad * x_rad;
    let x5: f64 = x3 * x_rad * x_rad;
    let x7: f64 = x5 * x_rad * x_rad;
    x_rad - (x3 / 6.0) + (x5 / 120.0) - (x7 / 5040.0)
}

pub fn cos(x: f64) -> f64 {
    let x_rad: f64 = x * core::f64::consts::PI / 180.0;
    let x2: f64 = x_rad * x_rad;
    let x4: f64 = x2 * x2;
    let x6: f64 = x4 * x_rad * x_rad;
    1.0 - (x2 / 2.0) + (x4 / 24.0) - (x6 / 720.0)
}

pub fn tan(x: f64) -> f64 {
    sin(x) / cos(x)
}

pub fn cot(x: f64) -> f64 {
    1.0 / tan(x)
}

pub fn sec(x: f64) -> f64 {
    1.0 / cos(x)
}

pub fn csc(x: f64) -> f64 {
    1.0 / sin(x)
}

pub fn arcsin(x: f64) -> f64 {
    let x3: f64 = x * x * x;
    let x5: f64 = x3 * x * x;
    let x7: f64 = x5 * x * x;
    let x9: f64 = x7 * x * x;
    x + (x3 / 6.0) + (3.0 * x5 / 40.0) + (5.0 * x7 / 112.0) + (35.0 * x9 / 1152.0)
}

pub fn arccos(x: f64) -> f64 {
    core::f64::consts::PI / 2.0 - arcsin(x)
}

pub fn arctan(x: f64) -> f64 {
    let x2: f64 = x * x;
    let x3: f64 = x * x2;
    let x5: f64 = x3 * x2;
    let x7: f64 = x5 * x2;
    let x9: f64 = x7 * x2;
    x - x3 / 3.0 + x5 / 5.0 - x7 / 7.0 + x9 / 9.0
}

pub fn arccot(x: f64) -> f64 {
    core::f64::consts::PI / 2.0 - arctan(x)
}

pub fn arcsec(x: f64) -> f64 {
    arccos(1.0 / x)
}

pub fn arccsc(x: f64) -> f64 {
    arcsin(1.0 / x)
}

pub fn sinh(x: f64) -> f64 {
    (exp(x) - exp(-x)) / 2.0
}

pub fn cosh(x: f64) -> f64 {
    (exp(x) + exp(-x)) / 2.0
}

pub fn tanh(x: f64) -> f64 {
    let epx: f64 = exp(x);
    let enx: f64 = exp(-x);
    (epx - enx) / (epx + enx)
}

pub fn ln(x: f64) -> f64 {
    let mut result: f64 = 0.0;
    let mut term: f64 = (x - 1.0) / (x + 1.0);
    let sqr_term: f64 = term * term;
    let mut n = 1.0;
    while n < 100.0 {
        result += term / n;
        term *= sqr_term;
        n += 2.0;
    }
    result * 2.0
}

pub fn log2(x: f64) -> f64 {
    ln(x) / ln(2.0)
}

pub fn log10(x: f64) -> f64 {
    ln(x) / ln(10.0)
}

pub fn log(base: f64, x: f64) -> f64 {
    ln(x) / ln(base)
}

pub fn is_prime(n: i64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    let mut i: i64 = 5;
    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let t: i64 = b;
        b = a % b;
        a = t;
    }
    abs(a)
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

pub fn sum(numbers: &[f64]) -> f64 {
    let mut total: f64 = 0.0;
    for &number in numbers {
        total += number;
    }
    total
}

pub fn difference(numbers: &[f64]) -> f64 {
    numbers.iter().fold(0.0, |acc, &x| acc - x)
}

pub fn product(numbers: &[f64]) -> f64 {
    numbers.iter().fold(1.0, |acc, &x| acc * x)
}

pub fn division(numbers: &[f64]) -> f64 {
    numbers.iter().fold(1.0, |acc, &x| acc / x)
}

pub fn max_value(numbers: &[f64]) -> f64 {
    let mut max: f64 = numbers[0];
    for &number in &numbers[1..] {
        if number > max {
            max = number;
        }
    }
    max
}

pub fn min_value(numbers: &[f64]) -> f64 {
    let mut min: f64 = numbers[0];
    for &number in &numbers[1..] {
        if number < min {
            min = number;
        }
    }
    min
}

pub fn mode(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return f64::NAN;
    }
    let mut max_element: f64 = numbers[0];
    let mut max_count: i32 = 1;
    for i in 0..numbers.len() {
        let mut count: i32 = 0;
        for j in 0..numbers.len() {
            if numbers[j] == numbers[i] {
                count += 1;
            }
        }
        if count > max_count {
            max_count = count;
            max_element = numbers[i];
        }
    }
    max_element
}

pub fn median(numbers: &mut [f64]) -> f64 {
    if numbers.is_empty() {
        return f64::NAN;
    }
    let len: usize = numbers.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
    let mid: usize = len / 2;
    if len % 2 == 0 {
        (numbers[mid - 1] + numbers[mid]) / 2.0
    } else {
        numbers[mid]
    }
}

pub fn range(numbers: &[f64]) -> f64 {
    let max = max_value(numbers);
    let min = min_value(numbers);
    max - min
}

pub fn mean(numbers: &[f64]) -> f64 {
    sum(numbers) / numbers.len() as f64
}

pub fn geometric_mean(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return f64::NAN;
    }
    let mut log_sum: f64 = 0.0;
    for &number in numbers {
        log_sum += ln(number);
    }
    let mean_log: f64 = log_sum / numbers.len() as f64;
    exp(mean_log)
}

pub fn harmonic_mean(numbers: &[f64]) -> f64 {
    if numbers.is_empty() {
        return f64::NAN;
    }
    let mut sum_reciprocals: f64 = 0.0;
    for &x in numbers {
        if x == 0.0 {
            return f64::INFINITY;
        }
        sum_reciprocals += 1.0 / x;
    }
    numbers.len() as f64 / sum_reciprocals
}

pub fn weighted_mean(numbers: &[f64], weights: &[f64]) -> f64 {
    if numbers.is_empty() || weights.len() != numbers.len() {
        return f64::NAN;
    }
    let mut weighted_sum: f64 = 0.0;
    let mut sum_weights: f64 = 0.0;
    for (&x, &w) in numbers.iter().zip(weights) {
        weighted_sum += x * w;
        sum_weights += w;
    }
    if sum_weights == 0.0 {
        return f64::NAN;
    }
    weighted_sum / sum_weights
}

pub fn trimmed_mean(numbers: &mut [f64], percent: f64) -> f64 {
    if numbers.is_empty() || percent >= 50.0 {
        return f64::NAN;
    }
    let len: usize = numbers.len();
    for i in 0..len {
        for j in 0..len - i - 1 {
            if numbers[j] > numbers[j + 1] {
                numbers.swap(j, j + 1);
            }
        }
    }
    let trim_count: usize = round((len as f64) * percent / 100.0) as usize;
    let valid_numbers: &[f64] = &numbers[trim_count..len - trim_count];
    let mut sum: f64 = 0.0;
    for &num in valid_numbers {
        sum += num;
    }
    sum / valid_numbers.len() as f64
}

pub fn polynomial(x: f64, n: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for (i, &num) in n.iter().enumerate() {
        result += num * pow(x, i as i32);
    }
    result
}

pub fn vector_add(v1: &[f64], v2: &[f64], result: &mut [f64]) {
    for i in 0..v1.len() {
        result[i] = v1[i] + v2[i];
    }
}

pub fn vector_sub(v1: &[f64], v2: &[f64], result: &mut [f64]) {
    for i in 0..v1.len() {
        result[i] = v1[i] - v2[i];
    }
}

pub fn dot_product(v1: &[f64], v2: &[f64]) -> f64 {
    let mut result: f64 = 0.0;
    for i in 0..v1.len() {
        result += v1[i] * v2[i];
    }
    result
}

pub fn cross_product(v1: &[f64], v2: &[f64], result: &mut [f64]) {
    result[0] = v1[1] * v2[2] - v1[2] * v2[1];
    result[1] = v1[2] * v2[0] - v1[0] * v2[2];
    result[2] = v1[0] * v2[1] - v1[1] * v2[0];
}

pub fn bessel_j0(x: f64) -> f64 {
    let mut sum: f64 = 1.0;
    let mut term: f64 = 1.0;
    let mut m: f64 = 1.0;
    while fabs(term) > 1e-10 {
        term *= -x * x / (4.0 * m * m);
        sum += term;
        m += 1.0;
    }
    sum
}

pub fn gamma(x: f64) -> f64 {
    if x < 0.5 {
        core::f64::consts::PI / (sin(core::f64::consts::PI * x) * gamma(1.0 - x))
    } else {
        let z: f64 = x - 1.0;
        let t: f64 = z + 7.5;
        (pow(t, (z + 0.5) as i32) * exp(-t)) * (2.5066282746310005 * (1.0 + 1.0 / (12.0 * x) - 1.0 / (360.0 * pow(x, 3))))
    }
}

pub fn erf(x: f64) -> f64 {
    let mut sum: f64 = 0.0;
    let mut term: f64 = x;
    let mut n: f64 = 1.0;
    while fabs(term) > 1e-10 {
        sum += term;
        term *= -x * x / n;
        n += 2.0;
    }
    sum * 2.0 / (sqrt(core::f64::consts::PI))
}

pub fn complex_add(a_real: f64, a_imag: f64, b_real: f64, b_imag: f64) -> (f64, f64) {
    (a_real + b_real, a_imag + b_imag)
}

pub fn complex_sub(a_real: f64, a_imag: f64, b_real: f64, b_imag: f64) -> (f64, f64) {
    (a_real - b_real, a_imag - b_imag)
}

pub fn complex_mul(a_real: f64, a_imag: f64, b_real: f64, b_imag: f64) -> (f64, f64) {
    (a_real * b_real - a_imag * b_imag, a_real * b_imag + a_imag * b_real)
}

pub fn complex_div(a_real: f64, a_imag: f64, b_real: f64, b_imag: f64) -> (f64, f64) {
    let denominator: f64 = b_real * b_real + b_imag * b_imag;
    ((a_real * b_real + a_imag * b_imag) / denominator, (a_imag * b_real - a_real * b_imag) / denominator)
}

pub fn complex_conjugate(a_real: f64, a_imag: f64) -> (f64, f64) {
    (a_real, -a_imag)
}