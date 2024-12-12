use rayon::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::time::Instant;

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn blink(mut rocks: HashMap<u128, u128>) -> u128 {
    for _ in 0..75 {
        let mut new_rocks: HashMap<u128, u128> = HashMap::new();
        for n in &rocks {
            let number = n.0.to_string();
            if number.len() % 2 == 0 {
                let middle = number.len() / 2;
                *new_rocks
                    .entry(number[middle..].parse::<u128>().expect("Bad parsing"))
                    .or_insert(0) += *n.1;
                *new_rocks
                    .entry(
                        number[..middle]
                            .parse::<u128>()
                            .expect("Bad parse in workload"),
                    )
                    .or_insert(0) += *n.1;
            } else if *n.0 == 0 {
                *new_rocks.entry(1).or_insert(0) += rocks.get(&0).unwrap();
            } else {
                *new_rocks.entry(*n.0 * 2024).or_insert(0) += rocks.get(n.0).unwrap();
            }
        }
        rocks = new_rocks;
    }

    let mut rock_count: u128 = 0;
    for rock in rocks {
        rock_count += rock.1;
    }
    rock_count
}

fn main() {
    let start = Instant::now();
    let input = read_file("./input.txt")
        .expect("Failed to read")
        .next()
        .expect("Failed to get input")
        .expect("Failed to get line");

    let line: HashMap<u128, u128> = input
        .split(" ")
        .map(|str| (str.parse::<u128>().expect("Bad Parse"), 1))
        .collect::<HashMap<u128, u128>>();

    println!("Line {:?}", line);

    let handles: Vec<u128> = line
        .par_iter()
        .map(|rock| blink(HashMap::from([(*rock.0, *rock.1)])))
        .collect();

    let mut answer = 0;
    for handle in handles {
        answer += handle;
    }
    println!("Rock Count {}", answer);
    let duration = start.elapsed();
    println!("Code took {:?} to execute.", duration);
}
