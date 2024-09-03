pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

pub fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

pub fn unsafe_divide(a: i32, b: i32) -> i32 {
    a / b
}

pub fn divide(a: i32, b: i32) -> Option<i32> {
    if b == 0 {
        Option::None
    } else {
        Option::Some(unsafe_divide(a, b))
    }
}

pub fn factorial(n: u32) -> u32 {
    let mut result = 1;
    let mut i = 1;
    while i <= n {
        result *= i;
        i += 1;
    };
    result
}

pub fn power(base: i32, exponent: u32) -> i32 {
    let mut result = 1;
    let mut exp = exponent;
    while exp > 0 {
        result *= base;
        exp -= 1;
    };
    result
}

pub fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

pub fn is_prime(n: u32) -> bool {
    if n <= 1 {
        return false;
    }
    let mut i = 2;
    let mut result = true;
    while i < n {
        if n % i == 0 {
            result = false;
        }
        i += 1;
    };
    result
}
