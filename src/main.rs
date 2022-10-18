use std::io;
mod specification;
mod contact;
mod quality;
mod maintenance;
mod equipment;
mod application;
mod cost;

fn main() {
    loop {
        intro_function();
        let mut user_value = String::new();
        io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line.");
        let user_value: i32 = user_value.trim().parse().expect("Please type a number!");

        if user_value == 9 {
            specification::usaf_201027456::info_spec_print();
        } else if user_value == 1 {
            get_tank_dimension_function();
        } else if user_value == 2 {
            get_solution_makeup_function();
        } else if user_value == 3 {
            application::application::application();
        } else if user_value == 4 {
            contact::contact::contact();
        } else if user_value == 5 {
            quality::quality::quality();
        } else if user_value == 6 {
            equipment::equipment::equipment();
        } else if user_value == 7 {
            maintenance::maintenance::maintenance();
        } else if user_value == 8 {
            cost::cost::cost();
        } else {
            println!("You entered an invalid respond.");
        }
        println!(
            "\nDo you want to review another selection for plating Zn-Ni: Y for yes and N for no"
        );
        let mut user_continue = String::new();
        io::stdin()
            .read_line(&mut user_continue)
            .expect("Failed to read line.");
        let user_continue = user_continue.trim();
        if user_continue == "N" {
            break println!("Thanks for using the app, Bye");
        }
    }
}

fn get_tank_dimension_function() {
    println!("\nPlease enter height dimension in inches of the tank: ");
    let mut tank_height = String::new();
    io::stdin()
        .read_line(&mut tank_height)
        .expect("Failed to read line");
    let tank_height: f32 = tank_height.trim().parse().expect("Please type a number!");

    println!("Please enter width dimension in inches of the tank: ");
    let mut tank_width = String::new();
    io::stdin()
        .read_line(&mut tank_width)
        .expect("Failed to read line");
    let tank_width: f32 = tank_width.trim().parse().expect("Please type a number!");

    println!("Please enter length dimension in inches of the tank: ");
    let mut tank_length = String::new();
    io::stdin()
        .read_line(&mut tank_length)
        .expect("Failed to read line");
    let tank_length: f32 = tank_length.trim().parse().expect("Please type a number!");

    println!("Please enter filled depth dimension in inches of the tank: ");
    let mut tank_depth = String::new();
    io::stdin()
        .read_line(&mut tank_depth)
        .expect("Failed to read line");
    let tank_depth: f32 = tank_depth.trim().parse().expect("Please type a number!");

    let full_volume: f32 = tank_height * tank_width * tank_length * 0.004329;
    let less_volume: f32 = (tank_height - tank_depth) * tank_width * tank_length * 0.004329;
    let fill_volume: f32 = full_volume - less_volume;

    println!("{}, {}, {}", tank_height, tank_length, tank_width);
    println!(
        "Total Tank Capacity volume is {} gallons ",
        full_volume.floor()
    );
    println!("Filled Tank volume is {} gallons ", fill_volume.floor());
}

fn get_solution_makeup_function() {
    let fill_volume: f32 = 1.0;
    println!("Please enter solution: 1 for Dispol IZ-C17 or 2 for Dispol IZ-264");
    let mut user_chemical = String::new();
    io::stdin()
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


fn intro_function() {
    println!("This program is for setting up Zinc-Nickel Plating as a");
    println!("low hydrogen embrittlement alkaline electrodeposited");

    println!("Enter a value to review selections below: ");
    println!("Enter 1 to calculate tank size in gallons");
    println!("Enter 2 to see amount of chemicals required in gallons and pounds");
    println!("Enter 3 to see Application of Zinc-Nickel Plating and Trivalent Chromate Conversion");
    println!("Enter 4 to see contacts from chemical vendors");
    println!("Enter 5 to see Quality Assurance Requirements");
    println!("Enter 6 to see Equipment Requirements");
    println!("Enter 7 to see Maintance Requirements");
    println!("Enter 8 to see cost of chemicals");
    println!("Enter 9 to see met specifications");
}
