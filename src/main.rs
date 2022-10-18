use std::io;
mod application;
mod contact;
mod cost;
mod equipment;
mod intro;
mod maintenance;
mod makeup;
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
            makeup::makeup::makeup();
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
