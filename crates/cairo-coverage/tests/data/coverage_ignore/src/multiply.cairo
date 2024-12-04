pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[cfg(test)]
mod tests {
    use super::multiply;

    #[test]
    fn it_works() {
        assert(multiply(2, 1) == 2, '');
    }
}
