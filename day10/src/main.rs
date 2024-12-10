use std::collections::HashSet;
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

fn find_trailheads(map: &[Vec<i8>]) -> Vec<(usize, usize)> {
    let mut trailheads: Vec<(usize, usize)> = Vec::new();

    for (y, line) in map.iter().enumerate() {
        for (x, number) in line.iter().enumerate() {
            if *number == 0 {
                trailheads.push((x, y));
            }
        }
    }
    trailheads
}

fn score(
    map: &[Vec<i8>],
    pos: (usize, usize),
    mut trails: HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    if map[pos.1][pos.0] == 9 {
        trails.insert((pos.0, pos.1));
        return trails;
    }
    // ^
    if pos.1 > 0 && map[pos.1 - 1][pos.0] == map[pos.1][pos.0] + 1 {
        trails = score(map, (pos.0, pos.1 - 1), trails);
    }
    // v
    if pos.1 < map.len() - 1 && map[pos.1 + 1][pos.0] == map[pos.1][pos.0] + 1 {
        trails = score(map, (pos.0, pos.1 + 1), trails);
    }
    // <
    if pos.0 > 0 && map[pos.1][pos.0 - 1] == map[pos.1][pos.0] + 1 {
        trails = score(map, (pos.0 - 1, pos.1), trails);
    }
    // >
    if pos.0 < map[pos.1].len() - 1 && map[pos.1][pos.0 + 1] == map[pos.1][pos.0] + 1 {
        trails = score(map, (pos.0 + 1, pos.1), trails);
    }
    trails
}

fn main() {
    let input = read_file("./input.txt").expect("Failed to read input");

    let map: Vec<Vec<i8>> = input
        .map(|line| {
            line.expect("Line not good")
                .chars()
                .map(|n| n.to_digit(10).expect("Not a value") as i8)
                .collect()
        })
        .collect();

    let trailheads: Vec<(usize, usize)> = find_trailheads(&map);

    let mut answer = 0;
    for start in trailheads {
        let trailhead_ends = score(&map, start, HashSet::new());
        // println!("Score of trail {:?} is {:?}", start, trailhead_ends);
        answer += trailhead_ends.len();
    }

    println!("Score {}", answer);

    // println!("Trailheads {:?}", trailheads);
    // for line in map {
    //     println!("{:?}", line);
    // }
}
