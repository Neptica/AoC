use std::fs::File;
use std::io::{self, BufRead};

fn read_files(file_name: &str) -> Result<Vec<Vec<i64>>, std::io::Error> {
    let file = File::open(file_name)?; // Open the file
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let line = line?; // Handle errors with '?'

        let levels: Result<Vec<i64>, _> = line
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<i64>())
            .collect();

        match levels {
            Ok(v) => reports.push(v),
            Err(e) => println!("{}", e),
        }
    }
    Ok(reports)
}

fn main() {
    let mut answer: i64 = 0;
    if let Ok(r) = read_files("./reports.txt") {
        for report in r {
            let mut ascending = false;
            let mut descending = false;
            let mut skipped = false;

            let mut valid = true;

            for index in 0..report.len() - 1 {
                if report[index] < report[index + 1] {
                    if index == 1 && descending {
                        descending = false;
                        skipped = true;
                        break;
                    } else {
                        ascending = true;
                    }
                } else if report[index] > report[index + 1] {
                    if index == 1 && ascending {
                        ascending = false;
                        skipped = true;
                        break;
                    } else {
                        descending = true;
                    }
                } else {
                    // When they are equal
                    if skipped == true {
                        valid = false;
                    } else {
                        skipped = true;
                    }
                    break;
                }

                // check for gap
                if report[index].abs_diff(report[index + 1]) > 3 {
                    // check if removing solves the problem
                    if index + 2 < report.len() && report[index].abs_diff(report[index + 2]) > 3 {
                        valid = false;
                        break;
                    } else {
                        skipped = true;
                        break;
                    }
                }
            }
            if valid {
                answer += 1;
            }
            println!("Report: {:?} \t Answer Count: {}", report, answer);
        }
    } else {
        println!("Error");
    }
    println!("{}", answer);
}
