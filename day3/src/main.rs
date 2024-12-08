use std::fs;

fn calc_ans(file_name: &str) -> Result<i64, std::io::Error> {
    let trim = fs::read_to_string(file_name)?;
    let input = trim.trim();

    let answer = find_multiplies(input);

    Ok(answer)
}

fn find_multiplies(string: &str) -> i64 {
    let mut total: i64 = 0;
    let mut l = 0;
    let mut do_mul = true;

    while l < string.len() {
        if let Some(current_char) = string.chars().nth(l) {
            if !("dm".contains(current_char)) {
                l += 1;
                continue;
            }
        }
        let mut r: usize = l;
        while let Some(r_char) = string.chars().nth(r) {
            if r_char == ')' {
                // check type first
                let word = &string[l..r + 1];
                if word == "do()" {
                    do_mul = true;
                    l = r + 1;
                    continue;
                } else if word == "don't()" {
                    do_mul = false;
                    l = r + 1;
                    continue;
                } else if l + 4 >= r + 1 {
                    l = r + 1;
                    break;
                } else if &word[0..4] == "mul(" {
                    // check validity
                    if check_match(&string[l + 4..r + 1]) && do_mul {
                        let group = &string[l + 4..r];
                        let numbers: Vec<i64> = group
                            .split(",")
                            .filter_map(|s| s.parse::<i64>().ok())
                            .collect();
                        if numbers.len() > 1 {
                            total += mul(numbers[0], numbers[1]);
                        }
                        l = r + 1;
                        break;
                    } else {
                        l += 1;
                        break;
                    }
                } else {
                    l += 4;
                    break;
                }
            // } else if r_char == '(' {
            //     break;
            } else {
                r += 1;
                if r == string.len() {
                    break;
                }
            }
        }
    }
    total
}

fn mul(a: i64, b: i64) -> i64 {
    a * b
}

fn check_match(text: &str) -> bool {
    let mut valid = true;
    let valid_chars = "),1234567890";
    let mut paren = false;
    let mut comma = false;
    for ch in text.chars() {
        if valid_chars.contains(ch) {
            if ch == ')' {
                if paren {
                    valid = false;
                    break;
                }
                paren = true;
            } else if ch == ',' {
                if comma {
                    valid = false;
                    break;
                }
                comma = true;
            }
            continue;
        } else {
            valid = false;
            break;
        }
    }
    valid
}

fn main() {
    if let Ok(ans) = calc_ans("./input.txt") {
        // if let Ok(ans) = calc_ans("./test.txt") {
        println!("Answer: {}", ans);
    } else {
        println!("Error");
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn teset_calc_ans() {
        assert_eq!(calc_ans("./test.txt").ok(), Some(483 * 583 + 281 * 2));
    }
}
