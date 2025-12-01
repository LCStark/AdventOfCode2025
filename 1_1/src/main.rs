use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let mut dial = 50;
    let mut zero_count = 0;
    
    for line in reader.lines() {
        let row = line.unwrap();
        let (character, num_string) = row.split_at(1);
        let number = num_string.parse::<i32>().unwrap();
        dial = match character {
            "L" => dial - number,
            _ => dial + number,
        };
        while dial < 0 {
            dial = dial + 100;
        }
        while dial > 99 {
            dial = dial - 100;
        }
        if dial == 0     {
            zero_count = zero_count + 1
        }
    }
    
    println!("Zero count: {zero_count}");    
    Ok(())
}
