pub fn makeup(_some_volume: f32) {
    use crate::readline::readline::read_line_int;

    println!("Please enter solution: 1 for Dispol IZ-C17 or 2 for Dispol IZ-264");
    let user_chemical = read_line_int();

    if user_chemical == 1 {
        println!("Require chemicals for Dispol IZ-C17 are as follows: ");
        println!(
            "Dipsol IZ-C17+MS : {:.1} gallons",
            _some_volume * 19.2 / 100.0
        );
        println!(
            "Dipsol Sodium Hydroxide: {:.1} pounds",
            _some_volume * 90.0 / 100.0
        );
        println!("Dipsol NZ-777: {:.1} gallons", _some_volume * 1.28 / 100.0);
        println!("Dipsol IZ-C17+B: {:.1} gallons", _some_volume * 2.1 / 100.0);
        println!("Dipsol F-0529: {:.1} gallons", _some_volume * 0.4 / 100.0);
    } else if user_chemical == 2 {
        println!("Require chemicals for Dispol IZ-264 are as follows: ");
        println!("Dispol IZ-264: {:.1} gallons", _some_volume * 7.5 / 100.0);
        println!("Dispol IZ-264 T: {:.1} gallons", _some_volume * 4.0 / 100.0);
    } else {
        println!("You entered an invalid respond!");
    }
}
