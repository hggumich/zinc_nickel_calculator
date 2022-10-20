pub fn dimension() -> f32 {
    use crate::readline::readline::read_line_float;

    struct TankSize {
        width: f32,
        length: f32,
        height: f32,
        fill: f32,
    }

    println!("\nPlease enter height dimension in inches of the tank: ");
    let tank_height = read_line_float();

    println!("Please enter width dimension in inches of the tank: ");
    let tank_width = read_line_float();

    println!("Please enter length dimension in inches of the tank: ");
    let tank_length = read_line_float();

    println!("Please enter fill height in inches of the tank: ");
    let tank_fill = read_line_float();

    let full_volume: f32 = tank_height * tank_width * tank_length * 0.004329;
    let less_volume: f32 = (tank_height - tank_fill) * tank_width * tank_length * 0.004329;
    let fill_volume: f32 = full_volume - less_volume;

    println!("{}, {}, {}", tank_height, tank_length, tank_width);
    println!("Total Tank Capacity volume is {:.2} gallons ", full_volume);
    println!("Filled Tank volume is {:.2} gallons ", fill_volume);

    fill_volume
}
