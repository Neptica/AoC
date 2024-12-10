use std::collections::VecDeque;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn read_file<P>(filename: P) -> io::Result<io::Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn main() {
    if let Ok(mut input) = read_file("./input.txt") {
        if let Some(line) = input.next() {
            let line: Vec<usize> = line
                .expect("Line not gotten")
                .chars() // .filter(|c| c.is_digit(10)) for weeding out non digit chars
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect();
            let mut disk: VecDeque<String> = VecDeque::new();

            for (index, digit) in line.into_iter().enumerate() {
                if index % 2 == 0 {
                    // id work
                    for _ in 0..digit {
                        disk.push_back((index / 2).to_string());
                    }
                } else {
                    // gap work
                    for _ in 0..digit {
                        disk.push_back('.'.to_string());
                    }
                }
            }
            // println!("Disk: {:?}", disk);

            let mut id: u128 = 0;
            let mut checksum: u128 = 0;

            while !disk.is_empty() {
                let mut first = disk.pop_front().expect("Bad front");
                while first == "." && !disk.is_empty() {
                    first = disk.pop_back().expect("Bad back");
                }
                if first == "." {
                    break;
                }
                // print!("{} ", first);
                let current_digit = first.parse::<u128>().expect("Bad Parse");
                checksum += id * current_digit;
                id += 1;
            }
            println!("Checksum: {}", checksum);

            // println!("{:?}", chars);
        }
    }
}
