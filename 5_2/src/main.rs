use std::fs::File;
use std::io::{prelude::*, BufReader};

fn read_range(line: &str) -> (u64, u64) {
    let (start, end) = line.split_once('-').unwrap();
    let start_index = start.parse::<u64>().unwrap();
    let end_index = end.parse::<u64>().unwrap();
    (start_index, end_index)
}

fn fold_overlapping(start_1: u64, end_1: u64, start_2: u64, end_2: u64) -> (u64, u64) {
    let start = match start_1 {
        s if s <= start_2 => s,
        _ => start_2
    };
    let end = match end_1 {
        e if e >= end_2 => e,
        _ => end_2,
    };
    
    (start, end)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);   
    let lines = reader.lines().into_iter();
    
    let mut ranges: Vec<(u64,u64)> = vec![];
    
    for line in lines {
        let current = line.unwrap();
        if current.len() == 0 {
            break;
        }
        ranges.push(read_range(&current));
    }
    ranges.sort();

    loop {        
        let mut folded_ranges = false;
    
        let mut unique_ranges: Vec<(u64, u64)> = vec![];
        
        'range: for range in ranges {
            for i in 0..unique_ranges.len() {
                if unique_ranges[i] == range { continue 'range; }
            
                if (range.0 >= unique_ranges[i].0 && range.0 <= unique_ranges[i].1) ||
                   (range.1 >= unique_ranges[i].0 && range.1 <= unique_ranges[i].1) {
                    let (start, end) = fold_overlapping(range.0, range.1, unique_ranges[i].0, unique_ranges[i].1);
                    unique_ranges[i] = (start, end);
                    folded_ranges = true;
                    continue 'range;
                }
            
            }           
            unique_ranges.push(range);
        }
        ranges = unique_ranges.clone();
        ranges.sort();
        
        if !folded_ranges { break; }
    }
    
    let mut fresh_count = 0;
    for range in &ranges {
        fresh_count += range.1 - range.0 + 1;
    }
    println!("Fresh count: {fresh_count}");
    
    Ok(())
}
