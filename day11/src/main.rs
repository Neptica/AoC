use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_file<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn action(mut rock: String, depth: u32) -> u128 {
    if depth >= 75 {
        return 1;
    }
    // print!("{} ", rock);

    if rock.len() % 2 == 0 {
        let middle = rock.len() / 2;
        let second_half = rock[middle..]
            .parse::<u64>()
            .expect("Bad Parse")
            .to_string(); // to
                          // remove leading zeros in the second value
        let first_half = rock[..middle].to_string();
        action(first_half, depth + 1) + action(second_half, depth + 1)
    } else if rock == "0" {
        rock = "1".to_string();
        return action(rock, depth + 1);
    } else {
        rock = (rock.parse::<u128>().expect("Parse error") * 2024).to_string();
        return action(rock, depth + 1);
    }
}

fn main() {
    let input = read_file("./input.txt")
        .expect("Failed to read")
        .next()
        .expect("Failed to get input")
        .expect("Failed to get line");

    let line: Vec<String> = input
        .split(" ")
        .map(|str| str.to_string())
        .collect::<Vec<String>>();

    println!("{:?}", line);

    let mut rock_count: u128 = 0;
    for rock in line {
        rock_count += action(rock, 0);
    }

    println!("Rock Count {}", rock_count);
}
