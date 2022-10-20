mod application;
mod contact;
mod cost;
mod equipment;
mod intro;
mod maintenance;
mod makeup;
mod print;
mod quality;
mod readline;
mod specification;
mod tank;

fn main() {
    let mut volume: f32 = 0.0;
    loop {
        intro::intro::intro();
        let user_value = readline::readline::read_line_int();

        match user_value {
            1 => volume = tank::dimension::dimension(),
            2 => makeup::makeup::makeup(volume),
            3 => application::application::application(),
            4 => contact::contact::contact(),
            5 => quality::quality::quality(),
            6 => equipment::equipment::equipment(),
            7 => maintenance::maintenance::maintenance(),
            8 => cost::cost::cost(),
            9 => specification::usaf_201027456::info_spec_print(),
            10 => print::print::print_pdf(),
            _ => println!("Invalid value entered!"),
        }

        println!(
            "\nDo you want to review another selection for plating Zn-Ni: Y for yes and N for no"
        );
        let user_continue = readline::readline::read_line_string();
        if user_continue == "N" || user_continue == "n" {
            break println!("Thanks for using the app, Bye");
        }
    }
}
