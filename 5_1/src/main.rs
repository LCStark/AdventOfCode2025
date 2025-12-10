use std::fs::File;
use std::io::{prelude::*, BufReader};

enum ReadingState {
    Ranges,
    Ingredients
}

fn read_range(line: &str) -> (u64, u64) {
    let (start, end) = line.split_once('-').unwrap();
    let start_index = start.parse::<u64>().unwrap();
    let end_index = end.parse::<u64>().unwrap();
    (start_index, end_index)
}

fn is_ingredient_fresh(id: u64, fresh_ranges: &Vec<(u64, u64)>) -> bool {
    for range in fresh_ranges {
        if id >= range.0 && id <= range.1 {
            return true;
        }
    }
    return false;
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);   
    let lines = reader.lines().into_iter();
    
    let mut state = ReadingState::Ranges;    
    let mut ranges: Vec<(u64,u64)> = vec![];
    let mut fresh_count = 0;
    
    for line in lines {
        let current = line.unwrap();
        match state {
            ReadingState::Ranges => {
                if current.len() == 0 {
                    state = ReadingState::Ingredients;
                    continue;
                }
                ranges.push(read_range(&current));
            },
            ReadingState::Ingredients => {
                if is_ingredient_fresh(current.parse::<u64>().unwrap(), &ranges) {
                    fresh_count += 1;
                }
            },
        };
    }
    
    println!("Fresh count: {0}", fresh_count);
    
    Ok(())
}
