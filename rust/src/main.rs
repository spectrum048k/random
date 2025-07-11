use rand::rngs::OsRng;
use rand::RngCore;
use textplots::{Chart, Plot, Shape};

const ARRAY_LENGTH: usize = 100;
const MIN_VALUE: u32 = 1;
const MAX_VALUE: u32 = 10;

fn main() {
    // Generate 100 cryptographically secure random integers between 1 and 10
    let mut rng = OsRng;
    let numbers: Vec<u32> = (0..ARRAY_LENGTH)
        .map(|_| (rng.next_u32() % (MAX_VALUE - MIN_VALUE + 1)) + MIN_VALUE)
        .collect();

    // Print the numbers
    println!("{:?}", numbers);

    // Prepare data for plotting (x, y) pairs
    let data: Vec<(f32, f32)> = numbers
        .iter()
        .enumerate()
        .map(|(i, &y)| (i as f32, y as f32))
        .collect();

    println!("\nRandom Numbers (Y: {}-{}, X: 1-{}):", MIN_VALUE, MAX_VALUE, ARRAY_LENGTH);
    Chart::new(100, 20, 0.0, ARRAY_LENGTH as f32)
        .lineplot(&Shape::Lines(&data))
        .display();
    println!("X-axis: 1 to {} (index of each random number)", ARRAY_LENGTH);
    println!("Y-axis: Value ({} to {})", MIN_VALUE, MAX_VALUE);
}
