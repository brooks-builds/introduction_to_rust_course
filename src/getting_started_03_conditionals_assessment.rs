// basic syntax of if
// expression if statements
// compiler will tell when we have to handle the else

// todo: finish writing tests for conditionals

fn is_greater_than(number: u8, tester: u8) -> bool {
    number > tester
}

fn add_five_if_less_than_0(number: i32) -> i32 {
    let result = if number < 0 { number + 5 } else { number };

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_greater_than() {
        assert!(!is_greater_than(5, 10));
        assert!(is_greater_than(11, 10));
        assert!(!is_greater_than(10, 10));
    }

    #[test]
    fn test_add_five_if_less_than_0() {
        assert_eq!(add_five_if_less_than_0(-3), 2);
        assert_eq!(add_five_if_less_than_0(0), 0);
        assert_eq!(add_five_if_less_than_0(1), 1);
    }
}
