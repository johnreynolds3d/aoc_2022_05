use itertools::Itertools;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input.txt")?;
    let lines = BufReader::new(f).lines();

    let mut matrix: Vec<Vec<char>> = Vec::with_capacity(9 * 8);

    for _ in 0..9 {
        matrix.push(vec![]);
    }

    for lines in &lines.chunks(8) {
        for line in lines.enumerate() {
            let my_string = line.1?;
            let mut j = 1;
            for i in 0..9 {
                matrix[i].insert(0, my_string.chars().nth(j).unwrap());
                j += 4;
            }
        }
        break;
    }
    println!("{:?}", matrix);

    Ok(())
}
