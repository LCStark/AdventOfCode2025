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

fn check_bank(bank: &str) -> (usize, usize) {
    let substr = bank.get(0 .. bank.len() - 1).unwrap();
    let first = get_max(substr);
    
    let substr2 = bank.get(first.unwrap() + 1 .. bank.len()).unwrap();
    let second = get_max(substr2);
    
    let first = first.unwrap();
    let second = second.unwrap() + first + 1;
    
    (first, second)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    
    for line in reader.lines() {
        let line = line.unwrap();
        let (first, second) = check_bank(line.clone().as_str());

        let num_string = format!("{0}{1}", line.chars().nth(first).unwrap(), line.chars().nth(second).unwrap());
        sum += num_string.parse::<i32>().unwrap();
    }
    
    println!("Sum: {sum}");
    
    Ok(())
}