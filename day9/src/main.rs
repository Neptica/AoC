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

            // part one
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

    // part two
    if let Ok(mut input) = read_file("./input.txt") {
        if let Some(line) = input.next() {
            let line: Vec<String> = line
                .expect("Line not gotten")
                .chars() // .filter(|c| c.is_digit(10)) for weeding out non digit chars
                .map(|c| c.to_string())
                .collect();

            let mut disk: Vec<(String, i32)> = line
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    if i % 2 == 0 {
                        ((i / 2).to_string(), s.parse::<i32>().expect("Failed parse"))
                    } else {
                        (".".to_string(), s.parse::<i32>().expect("Failed to parse"))
                    }
                })
                .collect();

            for tuple in &disk {
                print!("{:?} ", tuple);
            }
            println!();

            let mut disk_len = disk.len();
            let mut index = 0;
            while index < disk_len {
                if disk[index].0 == "." {
                    if disk[index].1 == 0 {
                        disk.remove(index);
                        disk_len -= 1;
                        continue;
                    }

                    let dot_count = disk[index].1;
                    for (i, back) in disk.clone().into_iter().enumerate().rev() {
                        if back.0 != "." && index < i && back.1 <= dot_count {
                            disk[index].1 -= back.1;
                            disk[i].0 = ".".to_string(); // change don't remove
                            if disk[index].1 == 0 {
                                disk.remove(index);
                                disk_len -= 1;
                            }
                            disk.insert(index, back);
                            break;
                        }
                    }
                }
                index += 1;
            }

            let mut id = 0;
            let mut new_checksum = 0;
            for tuple in disk {
                if tuple.0 != "." {
                    for _ in 0..tuple.1 {
                        new_checksum += tuple.0.parse::<u128>().expect("Failed to parse") * id;
                        id += 1;
                    }
                } else {
                    id += tuple.1 as u128;
                }
            }
            println!("New Checksum: {}", new_checksum);
        }
    }
}
