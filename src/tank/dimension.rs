use std::io;

pub fn dimension() {
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
