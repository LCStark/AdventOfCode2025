use std::fs::File;
use std::io::{prelude::*, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);   
    let lines:Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    
    let mut rows: Vec<Vec<String>> = vec![];
    
    for line in lines {
        let mut row: Vec<_> = line.split("").collect::<Vec<_>>();
        row = row[1..row.len()-1].to_vec();
        rows.push(row.iter().map(|l| l.to_string()).collect::<Vec<_>>());
    }
    
    
    let column_count = rows[0].len();
    let row_count = rows.len();
    
    let mut final_result = 0;
    
    let mut numbers: Vec<Vec<String>> = vec![];
    numbers.push(vec![]);
    let mut number_index = 0;
    
    'outer: for column in (0..column_count).rev() {
        for row in 0..row_count {
            let character = &rows[row][column];
            
            match character.as_str() {
                " " => continue,
                x if x == "+" || x == "*" => {                   
                    let problem_numbers: Vec<_> = numbers.iter().map(|n| n.join("").parse::<u64>().unwrap()).collect::<Vec<_>>();
                    
                    let result = match x {
                        "+" => problem_numbers.iter().sum(),
                        "*" => {
                            let mut product = problem_numbers[0];
                            for num in &problem_numbers[1..problem_numbers.len()] {
                                product = product * num;
                            }
                            product
                        },
                        _ => panic!("Unknown operation")
                    };
                    
                    final_result += result;
                    
                    numbers.clear();
                    number_index = 0;
                    numbers.push(vec![]);
                    continue 'outer;
                },
                _ => numbers[number_index].push(character.to_string())
            }
        }
        
        if numbers[number_index].len() > 0 {
            number_index += 1;
            numbers.push(vec![]);
        }
    }
    
    println!("Final result: `{:?}`", final_result);
    
    Ok(())
}
