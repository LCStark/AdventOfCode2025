use std::fs::File;
use std::io::{prelude::*, BufReader};

struct TurnResult {
    dial: i32,
    passes: i32
}

fn turn_dial(dial_current: i32, turn_distance: i32) -> TurnResult {
    let quotient = turn_distance / 100;
    let abs = quotient.abs();
    let remainder = turn_distance % 100;
    
    TurnResult{dial: dial_current + remainder, passes: abs}
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let mut dial = 50;
    let mut zero_count = 0;
        
    for line in reader.lines() {
        let row = line.unwrap();
        let (character, num_string) = row.split_at(1);
        let number = num_string.parse::<i32>().unwrap();
        let turn_result = match character {
            "L" => turn_dial(dial, -number),
            _ => turn_dial(dial, number),
        };
        
        zero_count = zero_count + turn_result.passes;
        
        dial = match turn_result.dial {
            d if d < 0 => {
                if dial != 0 {
                    zero_count = zero_count + 1;
                }
                d + 100
            },
            d if d > 99 => {
                if dial != 0 {
                    zero_count = zero_count + 1;
                }
                d - 100
            },
            _ => {
                if turn_result.dial == 0 {
                    zero_count = zero_count + 1;
                }
                turn_result.dial
            },
        };
    }
    
    println!("Zero count: {zero_count}");
    
    Ok(())
}
