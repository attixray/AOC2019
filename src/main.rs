use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let filename = r"C:\Users\Attila\Downloads\input1.txt";
    // Open the file in read-only mode (ignoring errors).
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);

    let mut sum = 0u32;
    // Read the file line by line using the lines() iterator from std::io::BufRead.
    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap(); // Ignore errors.
                                  // Show the line and its number.

        let value = line.parse::<u32>().unwrap();
        sum += (value / 3) - 2;
        println!("{0}. {1}", index + 1, line);
    }

    println!("sum: {}", sum);

    //println!("{}", contents);
    Ok(())
}
