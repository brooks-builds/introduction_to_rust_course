/// # Objectives
///
/// By the end of this lesson, you will be able to ...
///
/// * declare a function in Rust
/// * set the return type of a function
/// * return a value from a function in two ways
/// * take in a parameter
fn _main() {
    let _my_number = _returns_a_number();
    let _new_number = _add_ten(10);
}

fn _returns_a_number() -> u64 {
    // do stuff
    return 12;
}

fn _add_ten(number: i32) -> i32 {
    number + 10
}

fn _is_five(number: i32) -> bool {
    if number == 5 {
        true
    } else {
        false
    }
}
