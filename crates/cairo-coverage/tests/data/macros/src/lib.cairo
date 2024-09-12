fn function_with_macro() {
    assert!(1 == 1);
}

#[cfg(test)]
mod tests {
    use super::function_with_macro;

    #[test]
    fn function_with_macro_test() {
        function_with_macro()
    }
}
