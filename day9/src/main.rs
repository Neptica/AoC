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
            let mut id = 0;
            let mut disk: String = String::new();

            for (index, digit) in line.into_iter().enumerate() {
                if index % 2 == 0 {
                    // id work
                    let operand: String = id.to_string().repeat(digit);
                    disk.push_str(&operand);
                    id += 1;
                } else {
                    // gap work
                    let operand: String = '.'.to_string().repeat(digit);
                    disk.push_str(&operand);
                }
            }
            // println!("{}", disk);

            let mut chars: Vec<char> = disk.chars().collect();
            let mut index: usize = 0;

            while index < chars.len() {
                let c = chars[index];
                if c == '.' {
                    if let Some(mut last) = chars.pop() {
                        while last == '.' && index < chars.len() {
                            last = chars.pop().expect("Bad last");
                        }
                        if index >= chars.len() {
                            if last != '.' {
                                chars.push(last);
                            }
                            break;
                        }
                        let _remove = chars.remove(index);
                        chars.insert(index, last);
                        // if remove != '.' {
                        //     println!("Removing {}", remove);
                        // }
                    }
                }
                index += 1;
            }

            // println!("{:?}", chars);

            let mut checksum: u128 = 0;
            for (i, c) in chars.into_iter().enumerate() {
                if c == '.' {
                    println!("Dumb period");
                    break;
                }
                let current_digit = (c as u8 - b'0') as u128;
                checksum += i as u128 * current_digit;
            }
            println!("Checksum: {}", checksum);
        }
    }
}
