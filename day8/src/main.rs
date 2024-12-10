use std::collections::HashMap;
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

fn main() {
    // if let Ok(input) = read_file("./test.txt") {
    if let Ok(input) = read_file("./input.txt") {
        let lines = input
            .collect::<io::Result<Vec<String>>>()
            .expect("Failed lines creation");

        let mut map: HashMap<u32, Vec<(i32, i32)>> = HashMap::new();
        for (y, line) in lines.clone().into_iter().enumerate() {
            for (x, char) in line.chars().enumerate() {
                if char != '.' {
                    map.entry(char as u32)
                        .or_default()
                        .push((x as i32, y as i32))
                }
            }
            // println!("{}", line);
        }

        // add each antinode even if its other counterpart is off the map
        // antinodes occur no matter if the area is occupied
        let mut antinodes: HashSet<(i32, i32)> = HashSet::new();
        let y_depth = lines.len();
        let x_depth = lines[0].len();

        for (key, values) in &map {
            let char_key = char::from_u32(*key).unwrap_or('?'); // Convert the u32 key back to a char
            println!("Key: '{}', Values: {:?}", char_key, values);

            for i in 0..values.len() {
                for j in i + 1..values.len() {
                    let first = values[i];
                    let second = values[j];

                    let dist1 = (first.0 - second.0, first.1 - second.1);
                    let dist2 = (second.0 - first.0, second.1 - first.1);

                    let antinode1 = (first.0 + dist1.0, first.1 + dist1.1);
                    let antinode2 = (second.0 + dist2.0, second.1 + dist2.1);

                    println!("{:?}, {:?}", antinode1, antinode2);

                    if antinode1.0 > -1
                        && antinode1.0 < x_depth as i32
                        && antinode1.1 > -1
                        && antinode1.1 < y_depth as i32
                    {
                        antinodes.insert(antinode1);
                    }
                    if antinode2.0 > -1
                        && antinode2.0 < x_depth as i32
                        && antinode2.1 > -1
                        && antinode2.1 < y_depth as i32
                    {
                        antinodes.insert(antinode2);
                    }
                }
            }
        }
        println!("Antinodes: {:?}", antinodes);
        println!("Antinode Count: {}", antinodes.len());
    }
}
