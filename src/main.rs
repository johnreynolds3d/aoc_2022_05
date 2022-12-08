use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut matrix: Vec<Vec<char>> = vec![
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
        vec![],
    ];

    let mut i = 0;

    for line in lines {
        if let Ok(line) = line {
            if i < 8 {
                for j in (1..35).step_by(4) {
                    matrix[i].insert(0, line.as_str().chars().nth(j).unwrap());
                }
                i += 1;
            }
        }
    }
    println!("{:?}", matrix);

    Ok(())
}
