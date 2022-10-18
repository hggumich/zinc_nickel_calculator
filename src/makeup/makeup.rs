pub fn makeup() {
    let fill_volume: f32 = 1.0;
    println!("Please enter solution: 1 for Dispol IZ-C17 or 2 for Dispol IZ-264");
    let mut user_chemical = String::new();
    std::io::stdin()
        .read_line(&mut user_chemical)
        .expect("Failed to read line");
    let user_chemical: i32 = user_chemical.trim().parse().expect("Please type a number!");

    if user_chemical == 1 {
        println!("Require chemicals for Dispol IZ-C17 are as follows: ");
        println!("Dipsol IZ-C17+MS : {} gallons", fill_volume * 19.2 / 100.0);
        println!(
            "Dipsol Sodium Hydroxide: {} pounds",
            fill_volume * 90.0 / 100.0
        );
        println!("Dipsol NZ-777: {} gallons", fill_volume * 1.28 / 100.0);
        println!("Dipsol IZ-C17+B: {} gallons", fill_volume * 2.1 / 100.0);
        println!("Dipsol F-0529: {} gallons", fill_volume * 0.4 / 100.0);
    } else if user_chemical == 2 {
        println!("Require chemicals for Dispol IZ-264 are as follows: ");
        println!("Dispol IZ-264: {} gallons", fill_volume * 7.5 / 100.0);
        println!("Dispol IZ-264 T: {} gallons", fill_volume * 4.0 / 100.0);
    } else {
        println!("You entered an invalid respond!");
    }
}
