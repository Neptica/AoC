use std::fs::File;
use std::io::{self, BufRead};

fn read_file(file_name: &str) -> Result<Vec<Vec<i64>>, std::io::Error> {
    println!("{}", file_name);
    let file = File::open(file_name)?; // Open the file
    let reader = io::BufReader::new(file);

    let mut reports: Vec<Vec<i64>> = Vec::new();
    for line in reader.lines() {
        let line = line?; // Handle errors with '?'

        let levels: Result<Vec<i64>, _> =
            line.split_whitespace().map(|s| s.parse::<i64>()).collect();

        match levels {
            Ok(v) => reports.push(v),
            Err(e) => println!("{}", e),
        }
    }
    Ok(reports)
}

fn check_safe(r: Vec<i64>) -> bool {
    let mut inc = true;
    let mut dec = true;
    let mut no_gaps = true;

    for index in 0..r.len() - 1 {
        if r[index] < r[index + 1] {
            dec = false;
        } else if r[index] > r[index + 1] {
            inc = false;
        }
        if r[index] == r[index + 1] || r[index].abs_diff(r[index + 1]) > 3 {
            no_gaps = false;
        }
    }
    (dec || inc) && no_gaps
}

fn check_removed(r: &Vec<i64>, index: i64) -> bool {
    let remove_index = index as usize;
    let mut new_vec: Vec<i64> = r.clone();
    if remove_index < new_vec.len() {
        new_vec.remove(remove_index);
        check_safe(new_vec)
    } else {
        false
    }
}

fn dampener_check(file: &str) -> i64 {
    if let Ok(reports) = read_file(file) {
        let mut now_safe = 0;
        for report in reports {
            // if check_safe(report.clone()) {
            //     now_safe += 1;
            //     continue;
            // }
            let mut inc_count = 0;
            let mut dec_count = 0;
            let mut gap_count = 0;
            let mut inc_index = -1;
            let mut dec_index = -1;
            let mut gap_index: i64 = -1;
            let last_index = report.len() - 1;
            for index in 0..report.len() - 1 {
                if report[index] < report[index + 1] {
                    inc_count += 1;
                    inc_index = index as i64;
                } else if report[index] > report[index + 1] {
                    dec_count += 1;
                    dec_index = index as i64;
                }
                if report[index].abs_diff(report[index + 1]) != 0
                    && report[index].abs_diff(report[index + 1]) < 4
                {
                    gap_count += 1;
                } else {
                    gap_index = index as i64;
                }
            }

            if (inc_count == last_index as i64 || dec_count == last_index as i64)
                && gap_count == last_index as i64
            {
                now_safe += 1;
            } else if inc_count == last_index as i64 - 1
                && (check_removed(&report, dec_index) || check_removed(&report, dec_index + 1))
            {
                now_safe += 1;
            } else if dec_count == last_index as i64 - 1
                && (check_removed(&report, inc_index) || check_removed(&report, inc_index + 1))
            {
                now_safe += 1;
            } else if gap_count == last_index as i64 - 1
                && (check_removed(&report, gap_index) || check_removed(&report, gap_index + 1))
            {
                now_safe += 1;
            } else {
                println!("Unsafe Report: {:?}", report);
            }
        }
        return now_safe;
    }
    0
}

fn main() {
    // part 1
    if let Ok(reports) = read_file("src/reports.txt") {
        let mut safe = 0;
        for report in reports {
            if check_safe(report) {
                safe += 1;
            }
        }
        println!("Safe: {}", safe);
    }

    // part 2
    let now_safe = dampener_check("src/reports.txt");
    println!("Now safe: {}", now_safe);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dampener_check() {
        assert_eq!(dampener_check("src/reporttest.txt"), 53);
    }
}
