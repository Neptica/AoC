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

fn create_vecs(filename: &str) -> Result<(Vec<i64>, Vec<Vec<i64>>), Box<dyn std::error::Error>> {
    let lines = read_file(filename)?;

    let mut totals: Vec<i64> = Vec::new();
    let mut numbers: Vec<Vec<i64>> = Vec::new();

    for line in lines {
        let line = line?;
        let mut parts = line.split(':');

        if let Some(total) = parts.next() {
            let total = total.trim().parse::<i64>()?;
            totals.push(total);
        }
        if let Some(number_group) = parts.next() {
            let number_group: Vec<i64> = number_group
                .split(' ')
                .filter_map(|n| n.trim().parse::<i64>().ok())
                .collect();
            numbers.push(number_group);
        }
    }

    Ok((totals, numbers))
}

fn can_make(target: i64, numbers: &[i64], working_number: i64) -> bool {
    if numbers.is_empty() {
        if working_number == target {
            return true;
        }
        return false;
    }

    can_make(target, &numbers[1..], working_number + numbers[0])
        || can_make(target, &numbers[1..], working_number * numbers[0])
        || {
            let new_working_number = format!("{}{}", working_number, numbers[0]);
            let current_working_number = new_working_number
                .parse::<i64>()
                .expect("Failed to join working number and next number");
            can_make(target, &numbers[1..], current_working_number)
        }
}

fn main() {
    // if let Ok((objective, numbers)) = create_vecs("./test.txt") {
    if let Ok((objective, numbers)) = create_vecs("./input.txt") {
        let mut answer = 0;
        for (count, objectiv) in objective.clone().into_iter().enumerate() {
            println!("{}: {:?}", objectiv, numbers[count]);
            let makes_target = can_make(objectiv, &numbers[count][1..], numbers[count][0]);
            if makes_target {
                answer += objectiv;
            }
        }
        println!("Total: {}", answer);
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn check_can_make() {
//         assert!(can_make(190, &vec![19][..], 10));
//         assert!(can_make(3267, &vec![40, 27][..], 81));
//         assert!(can_make(292, &vec![6, 16, 20][..], 11));
//         assert!(!can_make(83, &vec![5][..], 17));
//         assert!(!can_make(156, &vec![6][..], 15));
//         assert!(!can_make(7290, &vec![8, 6, 15][..], 6));
//         assert!(!can_make(161011, &vec![10, 13][..], 16));
//         assert!(!can_make(21037, &vec![7, 18, 13][..], 9));
//     }
// }
