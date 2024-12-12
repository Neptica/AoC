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

fn main() {
    let input = read_file("./input.txt")
        .expect("Failed to read")
        .next()
        .expect("Failed to get input")
        .expect("Failed to get line");

    let mut line: Vec<(u128, u64)> = input
        .split(" ")
        .map(|str| (str.parse::<u128>().expect("Bad Parse"), 1))
        .collect::<Vec<(u128, u64)>>();

    println!("{:?}", line);
    println!();
    for _ in 0..75 {
        let mut insert_vec: VecDeque<(usize, String, u64)> = VecDeque::new();
        for (i, n) in line.iter_mut().enumerate() {
            let number = n.0.to_string();
            if number.len() % 2 == 0 {
                let middle = number.len() / 2;
                let val: (usize, String, u64) = (
                    i,
                    number[middle..]
                        .parse::<u128>()
                        .expect("Bad parsing")
                        .to_string(),
                    n.1,
                ); // to
                   // remove leading zeros in the second value
                n.0 = number[..middle]
                    .parse::<u128>()
                    .expect("Bad parse in workload");
                insert_vec.push_back(val);
            } else if n.0 == 0 {
                n.0 = 1;
            } else {
                n.0 *= 2024;
            }
        }

        // modify the vector
        while !insert_vec.is_empty() {
            let (pos, val, multiplicity) = insert_vec.pop_back().expect("Pop vector bad news");
            line.insert(
                pos + 1,
                (
                    val.parse::<u128>()
                        .expect("Parsing gone wrong during injections"),
                    multiplicity,
                ),
            );
        }

        // find duplicates and shrink the vector
        let mut remove_indices: Vec<usize> = Vec::new();
        for i in 0..line.len() {
            if remove_indices.contains(&i) {
                continue;
            }
            for index in i + 1..line.len() {
                if remove_indices.contains(&index) {
                    continue;
                }
                if line[index].0 == line[i].0 {
                    line[i].1 += line[index].1;
                    remove_indices.push(index); // line[index] is now account for in multiplicity
                }
            }
        }

        remove_indices.sort();

        while let Some(loc) = remove_indices.pop() {
            line.remove(loc);
        }
    }

    println!("{:?}", line);
    let mut rock_count: u128 = 0;
    for rock in line {
        rock_count += rock.1 as u128;
    }
    println!("{:?}", rock_count);
}
