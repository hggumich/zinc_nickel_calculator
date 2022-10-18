pub fn dimension() {

    struct tank_size {
        width: f32,
        height: f32,
        depth: f32,
    }

    impl tank_size {
        fn print_info(&self){
            println!("Tank volume is {}", self.width*self.height*self*depth);
        }
    }


    println!("\nPlease enter height dimension in inches of the tank: ");
    let mut tank_height = String::new();
    std::io::stdin()
        .read_line(&mut tank_height)
        .expect("Failed to read line");
    let TankSize::height = tank_height.trim().parse().expect("Please type a number!");

    println!("Please enter width dimension in inches of the tank: ");
    let mut tank_width = String::new();
    std::io::stdin()
        .read_line(&mut tank_width)
        .expect("Failed to read line");
    let tank_width: f32 = tank_width.trim().parse().expect("Please type a number!");

    println!("Please enter length dimension in inches of the tank: ");
    let mut tank_length = String::new();
    std::io::stdin()
        .read_line(&mut tank_length)
        .expect("Failed to read line");
    let tank_length: f32 = tank_length.trim().parse().expect("Please type a number!");

    println!("Please enter filled depth dimension in inches of the tank: ");
    let mut tank_depth = String::new();
    std::io::stdin()
        .read_line(&mut tank_depth)
        .expect("Failed to read line");
    let tank_depth: f32 = tank_depth.trim().parse().expect("Please type a number!");

    let full_volume: f32 = TankSize::height * tank_width * tank_length * 0.004329;
    let less_volume: f32 = (TankSize::height - tank_depth) * tank_width * tank_length * 0.004329;
    let fill_volume: f32 = full_volume - less_volume;

    println!("{}, {}, {}", TankSize::height, tank_length, tank_width);
    println!(
        "Total Tank Capacity volume is {} gallons ",
        full_volume.floor()
    );
    println!("Filled Tank volume is {} gallons ", fill_volume.floor());
}
