/// We can create functions that don't return anything.
///
/// Create a function named `no_return` that doesn't return anything
fn no_return() {}

/// We can also create functions that return a type. For this challenge declare a function named `returns` and have it return a `i32` value of `25`.
fn returns() -> i32 {
    25
}

/// Functions can take in parameters which can be used in the function body. For this challenge create a function named `is_five` that will accept a single parameter of integer type and return a boolean. True if the number is equal to 5, false otherwise.
fn is_five(number: i32) -> bool {
    if number == 5 {
        return true;
    }

    false
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_no_return() {
        assert_eq!(no_return(), ());
    }

    #[test]
    fn test_returns() {
        let expected_result: i32 = 25;
        assert_eq!(returns(), expected_result);
    }

    #[test]
    fn test_is_five() {
        assert!(is_five(5));
        assert!(!is_five(7));
    }
}
