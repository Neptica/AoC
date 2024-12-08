use std::fs::File;
use std::io::{self, BufRead};

fn read_files(file_name: &str) -> Result<(Vec<i64>, Vec<i64>), io::Error> {
    let file = File::open(file_name)?; // Open the file
    let reader = io::BufReader::new(file);

    let mut list_id = Vec::new();
    let mut list_number = Vec::new();
    for line in reader.lines() {
        let line = line?; // Handle errors with '?'

        let parts: Vec<&str> = line.trim().split_whitespace().collect();

        if parts.len() >= 2 {
            if let Ok(first_num) = parts[0].parse::<i64>() {
                list_id.push(first_num);
            }
            if let Ok(second_num) = parts[1].parse::<i64>() {
                list_number.push(second_num);
            }
        }
    }
    Ok((list_id, list_number))
}

fn main() {
    let file_name: &str = "./input.txt";

    let mut list_ids: Vec<i64> = Vec::new();
    let mut list_numbers: Vec<i64> = Vec::new();

    match read_files(file_name) {
        Ok((first_numbers, second_numbers)) => {
            // Access vectors inside the Ok result
            list_ids = first_numbers;
            list_numbers = second_numbers;
        }
        Err(e) => {
            // Handle the error case
            eprintln!("Failed to read file: {}", e);
        }
    }
    list_ids.sort();
    list_numbers.sort();

    let mut numbers: Vec<Vec<i64>> = Vec::new();

    for number in &list_numbers {
        if let Some(last) = numbers.last_mut() {
            if last[0] == *number {
                last[1] += 1;
                // println!("Count {} for {}", last[1], last[0]);
            } else {
                numbers.push(vec![*number, 1]);
            }
        } else {
            numbers.push(vec![*number, 1]);
        }
        // if let Some(last) = numbers.last() {
        //     println!("unit {:?}", last);
        // }
    }

    let mut answer: i64 = 0;
    let mut number_pointer: usize = 0;

    for number in list_ids {
        let mut current_pointer = number_pointer;
        loop {
            if current_pointer < numbers.len() {
                if let parts = &numbers[current_pointer] {
                    println!("{} : {}", number, parts[0]);
                    if number == parts[0] {
                        answer += number * parts[1]; // number * count in other list
                        number_pointer = current_pointer + 1;
                        break;
                    } else if number > parts[0] {
                        current_pointer += 1;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }

    println!("Similarity score: {}", answer);
}
