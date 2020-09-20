/// # Objectives
///
/// By the end of this lesson, you will be able to ...
///
/// * declare a function in Rust
/// * set the return type of a function
/// * return a value from a function in two ways
/// * take in a parameter
fn _main() {
    let my_number = returns_a_number();
    let new_number = add_ten(10);
}

fn returns_a_number() -> u64 {
    // do stuff
    return 12;
}

fn add_ten(number: i32) -> i32 {
    number + 10
}

fn is_five(number: i32) -> bool {
    if number == 5 {
        true
    } else {
        false
    }
}
