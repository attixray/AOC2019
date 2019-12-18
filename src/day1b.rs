use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let filename = r"C:\Users\Attila\Downloads\input1.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0i32;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (_, line) in reader.lines().enumerate() {
        let mass = line.unwrap().parse::<u32>().unwrap();
        let mut fuel_for_mass = (mass / 3) as i32 - 2;
        while fuel_for_mass > 0 {
            sum += fuel_for_mass;
            fuel_for_mass = (fuel_for_mass / 3) - 2;
        }
    }

    println!("total mass of the rocket: {}", sum);

    Ok(())
}
