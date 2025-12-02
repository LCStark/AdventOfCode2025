use std::fs::File;
use std::io::{prelude::*, BufReader};

fn is_valid(id: &str) -> bool {
    let length = id.len();
    
    for i in 1..(length/2 + 1) {
        if length % i > 0 {
            continue;
        }
        let chunks:Vec<_> = id.chars().collect::<Vec<char>>().chunks(i).map(|c| c.iter().collect::<String>()).collect();
        
        let mut invalid = true;
        for n in 1..chunks.len() {
            if chunks[n - 1] != chunks[n] {
                invalid = false;
                break;
            }
        }
        
        if invalid {
            return false;
        }
    }
    
    true
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let data: String = reader.lines().map(|l| l.unwrap()).collect::<Vec<_>>().into_iter().collect();
    
    let data_vec: Vec<&str> = data.split(",").collect();
    
    let mut sum = 0;
    
    for range in data_vec {        
        let (start, end) = range.split_once("-").unwrap();
        
        for id in start.parse::<u64>().unwrap()..end.parse::<u64>().unwrap()+1 {
            if !is_valid(id.to_string().as_str()) {
                sum += id;
            }
        }
    }
    println!("Sum: {0}", sum);
    
    Ok(())
}
