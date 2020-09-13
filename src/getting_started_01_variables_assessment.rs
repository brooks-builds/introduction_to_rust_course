// This tells Rust to ignore the unused code warnings so that we don't clutter our editor with yellow squiggly lines
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables))]

/// Create a constant named `FIVE`, and set it to be an unsigned integer with a value of 5

/// Rust will figure out what the type should be because the function is declaring that it has a return type of a u64.
///
/// Make the test pass withoug declaring the type specifically on the variable declaration.
///
/// The test is expecting that the type returned is a i32
fn create_variable_without_setting_type() -> u64 {
    let my_variable = 55;

    my_variable
}

/// In Rust, variables are immutable by default, that means that they cannot have their values changed after declaration.
///
/// When we declare variables we can use the `mut` keyword to tell Rust that we want to be able to mutate the variable later.
///
/// Update the following function so that you can add 10 to result before it gets returned
fn mutable_variables() -> i32 {
    let result = 5;

    result
}

/// If we don't want to set a variable to be mutatable. Then we can shadow it to change it's value, or even it's type.
///
/// In this function the variable `result` is a u64 with the value 55.
///
/// Shadow `result` to set it to 23 before it gets returned
fn shadowing() -> i32 {
    let result: u64 = 55;

    result
}

/// Test declaration for all of the challenges in this file.
///
/// Don't change anything in here, but feel free to use it to understand what the tests are expecting and how things are working.
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_constant() {
        let five: u64 = 5;
        assert_eq!(FIVE, five);
    }

    #[test]
    fn test_create_variable_without_setting_type() {
        let result = create_variable_without_setting_type();
        let expected: i32 = 55;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_mutable_variables() {
        assert_eq!(mutable_variables(), 15);
    }

    #[test]
    fn test_shadowing() {
        let expected: i32 = 23;
        assert_eq!(shadowing(), expected);
    }
}
