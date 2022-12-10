use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut m: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        m.push(vec![]);
    }

    let mut i;
    let mut a = 0;
    let mut s: String;

    for line in BufReader::new(File::open("input.txt")?).lines() {
        s = line?;
        if a < 8 {
            i = 1;
            for j in (0..9).step_by(4) {
                m[j].insert(0, s.chars().nth(i).unwrap());
            }
            a += 1;
        }
        println!("{:?}", m);

        let v: Vec<_> = s.split(' ').collect();
        if v[0] == "move" {
            println!("{} {} {}", v[1], v[3], v[5]);
        }
    }

    Ok(())
}
