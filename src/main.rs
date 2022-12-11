use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        stacks.push(vec![]);
    }

    let mut i;
    let mut a = 0;
    let mut s;
    let mut qty: u8;
    let mut from: usize;
    let mut to: usize;

    for line in BufReader::new(File::open("input.txt")?).lines() {
        s = line?;
        if a < 8 {
            i = 0;
            for j in (1..35).step_by(4) {
                if s.chars().nth(j).unwrap() != ' ' {
                    stacks[i].insert(0, s.chars().nth(j).unwrap());
                }
                i += 1;
            }
            a += 1;
        }

        let v: Vec<_> = s.split(' ').collect();

        if v[0] == "move" {
            qty = v[1].parse().unwrap();
            from = v[3].parse().unwrap();
            to = v[5].parse().unwrap();

            // println!("{} {} {}", qty, from, to);

            // from -= 1;
            // to -= 1;

            // println!("stacks[from] before move: {:?}", stacks[from]);
            // println!("stacks[to] before move: {:?}", stacks[to]);

            for _ in 0..qty {
                let c = stacks[from - 1].pop().unwrap();
                stacks[to - 1].push(c);
            }

            // println!("stacks[from] after move: {:?}", stacks[from]);
            // println!("stacks[to] after move: {:?}", stacks[to]);
        }
    }
    for s in stacks {
        println!("{}", s[s.len() - 1]);
    }

    Ok(())
}
