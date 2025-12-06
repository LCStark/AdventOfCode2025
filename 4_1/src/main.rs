use std::fs::File;
use std::io::{prelude::*, BufReader};

fn count_neighbours(array: &Vec<Vec<char>>, x: i32, y: i32) -> i32 {  
    let mut neighbour_count = 0;
    for dy in -1..2 {
        let py = y + dy;
        if py < 0 || py > (array.len() - 1) as i32 { continue; }
        for dx in -1..2 {
            let px = x + dx;
            if px < 0 || px > (array[py as usize].len() - 1) as i32 { continue; }
            if px == x && py == y { continue; }
            
            if array[py as usize][px as usize] == '@' { neighbour_count += 1; }
        }
    }
    
    neighbour_count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("input")?;
    let reader = BufReader::new(file);
    
    let mut sum = 0;
    
    let mut array:Vec<Vec<char>> = vec![];
    
    for line in reader.lines() {
        let row:Vec<char> = line.unwrap().chars().collect();
        array.push(row);
    }
        
    for y in 0..array.len() {
        let row = &array[y];
        for x in 0..row.len() {
            if array[y as usize][x as usize] != '@' { continue; } 
            let count = count_neighbours(&array, x as i32, y as i32);
            if count < 4 { sum += 1; };
        }
    }
    
    println!("Sum: {0}", sum);
        
    Ok(())
}