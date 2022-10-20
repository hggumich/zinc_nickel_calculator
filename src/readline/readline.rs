pub fn read_line_float() -> f32 {
    let mut read_float = String::new();
    std::io::stdin()
        .read_line(&mut read_float)
        .expect("Failed to read line");
    read_float.trim().parse().expect("Please type a number!")
}

pub fn read_line_int() -> i32 {
    let mut read_int = String::new();
    std::io::stdin()
        .read_line(&mut read_int)
        .expect("Failed to read line");
    read_int.trim().parse().expect("Please type a number!")
}

pub fn read_line_string() -> String {
    let mut read_string = String::new();
    std::io::stdin()
        .read_line(&mut read_string)
        .expect("Failed to read line");
    read_string.trim().parse().expect("Please type a sting!")
}
