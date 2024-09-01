use complex_calculator::{add, subtract, multiply, divide, factorial, power, is_prime, fibonacci};

#[test]
fn test_add() {
    assert(add(2, 3) == 5, '');
}

#[test]
fn test_subtract() {
    assert(subtract(5, 3) == 2, '');
}

#[test]
fn test_multiply() {
    assert(multiply(4, 3) == 12, '');
}

#[test]
fn test_divide() {
    assert(divide(1, 0) == Option::None, '');
}

#[test]
fn test_factorial() {
    assert(factorial(5) == 120, '');
}

#[test]
fn test_power() {
    assert(power(2, 3) == 8, '');
    assert(power(5, 0) == 1, '');
}

#[test]
fn test_fibonacci() {
    assert(fibonacci(1) == 1, '');
}

#[test]
fn test_is_prime() {
    assert(is_prime(29), '');
    assert(!is_prime(15), '');
}
