use std::io;
mod application;
mod contact;
mod cost;
mod equipment;
mod intro;
mod maintenance;
mod quality;
mod specification;
mod tank;

fn main() {
    loop {
        intro::intro::intro();
        let mut user_value = String::new();
        io::stdin()
            .read_line(&mut user_value)
            .expect("Failed to read line.");
        let user_value: i32 = user_value.trim().parse().expect("Please type a number!");

        if user_value == 9 {
            specification::usaf_201027456::info_spec_print();
        } else if user_value == 1 {
            tank::dimension::dimension();
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
