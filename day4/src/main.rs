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

fn calc_ans(file_name: &str) -> i64 {
    let mut input = Vec::new();

    if let Ok(lines) = read_lines(file_name) {
        for line in lines.map_while(Result::ok) {
            input.push(line);
        }
    }
    let mut answer = 0;
    // answer += h_count(&input);
    // answer += v_count(input.clone());
    answer += slant(input); // slant now has ownership of input and input goes out of scope after
                            // this line

    answer
}

fn h_count(vector: &Vec<String>) -> i64 {
    let mut total: i64 = 0;
    let mut l = 0;
    let mut r = 4;
    let mut working_string: &str;
    for line in vector {
        while r < line.len() + 1 {
            working_string = &line[l..r];
            if working_string == "XMAS" || working_string == "SAMX" {
                total += 1;
            }
            l += 1;
            r += 1;
        }
        l = 0;
        r = 4;
    }
    total
}

fn v_count(vector: Vec<String>) -> i64 {
    let mut total: i64 = 0;
    if vector.len() < 4 {
        return total;
    }

    for index in 0..vector.len() - 3 {
        let mut pos = 0;
        let mut working_string: String;
        while pos < vector[0].len() {
            working_string = format!(
                "{}{}{}{}",
                vector[index].chars().nth(pos).unwrap_or('Z'),
                vector[index + 1].chars().nth(pos).unwrap_or('Z'),
                vector[index + 2].chars().nth(pos).unwrap_or('Z'),
                vector[index + 3].chars().nth(pos).unwrap_or('Z')
            );
            let _debug_string = working_string.to_string();

            if working_string == "XMAS" || working_string == "SAMX" {
                total += 1;
            }
            pos += 1;
        }
    }
    total
}

fn slant(vector: Vec<String>) -> i64 {
    let mut total: i64 = 0;
    if vector.len() < 3 {
        return total;
    }

    for index in 1..vector.len() - 1 {
        let mut pos = 1;
        let mut forward_string: String;
        let mut backward_string: String;
        while pos < vector[0].len() - 1 {
            if vector[index].chars().nth(pos).unwrap_or('Z') != 'A' {
                pos += 1;
                continue;
            }

            if pos < vector[0].len() - 1 {
                forward_string = format!(
                    "{}{}{}",
                    vector[index - 1].chars().nth(pos + 1).unwrap_or('Z'),
                    vector[index].chars().nth(pos).unwrap_or('Z'),
                    vector[index + 1].chars().nth(pos - 1).unwrap_or('Z'),
                );
                let _debug_string = forward_string.to_string();

                backward_string = format!(
                    "{}{}{}",
                    vector[index - 1].chars().nth(pos - 1).unwrap_or('Z'),
                    vector[index].chars().nth(pos).unwrap_or('Z'),
                    vector[index + 1].chars().nth(pos + 1).unwrap_or('Z'),
                );
                let _debug_string = backward_string.to_string();

                if (forward_string == "MAS" || forward_string == "SAM")
                    && (backward_string == "MAS" || backward_string == "SAM")
                {
                    println!("Both: {} : {}", index, pos);
                    total += 1;
                }
            }
            pos += 1;
        }
    }
    total
}

fn main() {
    let ans = calc_ans("./input.txt");
    println!("Answer: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_ans() {
        assert_eq!(calc_ans("./test.txt"), 1);
    }
}
