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
    let input = read_file("./test.txt").expect("Failed to read file");
    let map: Vec<Vec<char>> = input
        .map(|line| line.expect("Failed to get line").chars().collect())
        .collect();

    for line in map {
        println!("{:?}", line);
    }
}
