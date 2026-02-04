use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);   
    let lines:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    
    let mut rows: Vec<Vec<String>> = vec![];
    for line in lines {
        rows.push(line.split_whitespace().map(|l| l.to_string()).collect::<Vec<_>>());
    }
    
    let cols = rows[0].len();
    let mut final_result = 0;
      
    for i in 0..cols {
        let mut problem: Vec<_> = vec![];
        for j in 0..rows.len() {
            problem.push(rows[j][i].clone());
        }
        let operation = problem.pop().unwrap().parse::<char>().unwrap();
        
        let result = match operation {
            '+' => problem.iter().map(|x| x.parse::<u64>().unwrap()).sum(),
            '*' => {
                let nums: Vec<u64> = problem.iter().map(|x| x.parse::<u64>().unwrap()).collect();
                let mut product = nums[0];
                for num in &nums[1..nums.len()] {
                    product = product * num;
                }
                product
            },
            _ => panic!("Unknown operation")
        };
        final_result += result;
    }
    
    println!("Final result: {:?}", final_result);
    
    Ok(())
}
