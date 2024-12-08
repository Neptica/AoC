use std::char;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_guard(layout: Vec<Vec<char>>) -> (usize, usize) {
    let guard = ['^', 'v', '>', '<'];
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            if guard.contains(&layout[y][x]) {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn main() {
    if let Ok(map) = read_lines("./input.txt") {
        let mut direction = "Orientation";
        let guard_type = vec!['^', 'v', '>', '<'];

        let layout: Vec<Vec<char>> = map
            .filter_map(Result::ok)
            .map(|line| line.chars().collect())
            .collect();
        let (mut x, mut y) = find_guard(layout.clone());

        let guard = layout[y][x];
        match guard {
            '^' => direction = "up",
            'v' => direction = "down",
            '>' => direction = "left",
            '<' => direction = "right",
            _ => println!("Guard not actually found"),
        }

        println!(
            "pos of guard: {},{}; orientation of guard: {}",
            x, y, direction
        );
    }
}
