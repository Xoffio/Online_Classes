pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn is_even(number: i32) -> bool {
    match number % 2{
        0 => true, // number is even
        _ => false // number is odd
    }
}

#[cfg(test)]
mod tests {
    // Bring all upper functions into scope.
    use super::*;

    // Test that should pass
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4, "Expected 4; result was: {}", result);
        assert_ne!(result, 10);
    }

    // Test that should fail
    #[test]
    fn it_fails() {
        let result = add(2, 2);
        assert_eq!(result, 0, "(TEST) Expected 0; result was: {}", result);
    }

    // assert macro test
    #[test]
    fn test_is_even() {
        // do nothing if `true`
        // panic if `false`
        assert!(is_even(42));

        assert!(!is_even(3));
    }
}
