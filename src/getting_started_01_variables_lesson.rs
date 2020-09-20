const _TEN_NUMBER: i8 = 10;

pub fn lesson() {
    let mut my_variable_name = 55;
    println!("my variable name: {}", my_variable_name);
    let my_floating_variable_name = 55.0;
    dbg!(my_floating_variable_name);

    let _my_32_bit_floating_number: f32 = 55.0;

    let _my_number = get_number();

    my_variable_name += 1;

    let my_floating_variable_name: i64 = 64;
    dbg!(my_floating_variable_name);

    println!("my variable name: {}", my_variable_name);
}

fn get_number() -> f32 {
    23.0
}
