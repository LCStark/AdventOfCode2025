use std::fs::File;
use std::io::{prelude::*, BufReader};

fn get_max(bank: &str) -> Option<usize> {
    let chars = bank.chars().collect::<Vec<char>>();
        
    let mut index: Option<usize> = None;
    
    for i in 0..bank.len() {
        if index.is_some() && chars[i] <= chars[index.unwrap()] { continue; }      
        index = Some(i);
    }
    
    index
}

fn check_bank(bank: &str) -> String {
    let mut result = String::new();
    
    let mut left_limit = 0;
    
    for right_limit in (0..=11).rev() {
        let substr = bank.get(left_limit .. bank.len() - right_limit).unwrap();
        let max = get_max(substr);
        let index = max.unwrap() + left_limit;
        result.push(bank.chars().nth(index).unwrap());
        left_limit = index + 1;
    }
    
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        let num_string = check_bank(line.clone().as_str());
        sum += num_string.parse::<u64>().unwrap();
    }
    
    println!("Sum: {sum}");
    
    Ok(())
}