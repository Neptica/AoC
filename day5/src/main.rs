use std::assert_eq;
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

fn create_rules(lines: io::Lines<io::BufReader<File>>) -> (Vec<i32>, Vec<Vec<i32>>, Vec<Vec<i32>>) {
    let mut gathering_rules = true;
    let mut rules_set: Vec<Vec<i32>> = Vec::new();
    let mut number_guide: Vec<i32> = Vec::new();
    let mut lists: Vec<Vec<i32>> = Vec::new();

    for line in lines {
        match line {
            Ok(line) => {
                if line.is_empty() {
                    gathering_rules = false;
                    continue;
                }

                if gathering_rules {
                    let parts: Vec<i32> = line
                        .split('|')
                        .map(|s| s.parse::<i32>().expect("Failed to parse"))
                        .collect();

                    if number_guide.is_empty() {
                        number_guide.push(parts[0]);
                        rules_set.push(vec![parts[1]]);
                    } else if let Ok(pos) = number_guide.binary_search(&parts[0]) {
                        rules_set[pos].push(parts[1]);
                    } else {
                        let mut inserted = false;
                        for (index, value) in number_guide.iter().enumerate() {
                            if parts[0] < *value {
                                number_guide.insert(index, parts[0]);
                                rules_set.insert(index, vec![parts[1]]);
                                inserted = true;
                                break;
                            }
                        }

                        if !inserted {
                            number_guide.push(parts[0]);
                            rules_set.push(vec![parts[1]]);
                        }
                    }
                } else {
                    let parts: Vec<i32> = line
                        .split(',')
                        .map(|s| s.parse::<i32>().expect("Failed to parse"))
                        .collect();
                    lists.push(parts);
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    (number_guide, rules_set, lists)
}

fn calc_ans(file: &str) -> i32 {
    if let Ok(input) = read_lines(file) {
        let (guides, rules, lists) = create_rules(input);
        for i in 0..guides.len() {
            println!("{}: ({}, {:?})", i, guides[i], rules[i]);
        }
        let mut ans = 0;

        for mut list in lists {
            let mut complete = false;
            let mut pos = 1;
            loop {
                let mut moved = false;
                for index in pos..list.len() {
                    if let Ok(rule_index) = guides.binary_search(&list[index]) {
                        for cindex in 0..index {
                            if rules[rule_index].contains(&list[cindex]) {
                                let current = list.remove(index);
                                list.insert(cindex, current);
                                pos = cindex + 1;
                                moved = true;
                                break;
                            }
                        }
                        if moved {
                            break;
                        }
                    }
                    if index == list.len() - 1 {
                        complete = true;
                    }
                }
                if complete {
                    break;
                }
            }

            let middle = (list.len() - 1) / 2;
            ans += list[middle];

            // let mut proper: Vec<i32> = Vec::new();
            // for number in list {
            //     if proper.is_empty() {
            //         proper.push(number);
            //     } else if let Ok(rule_index) = guides.binary_search(&number) {
            //         let mut inserted = false;
            //         for (index, value) in proper.iter().enumerate() {
            //             if rules[rule_index].contains(value) {
            //                 proper.insert(index, number);
            //                 inserted = true;
            //                 break;
            //             }
            //         }
            //         if !inserted {
            //             proper.push(number);
            //         }
            //     } else {
            //         proper.push(number);
            //     }
            // }
        }

        println!("Sum of the Middle Element of Proper Lists: {}", ans);
        ans
    } else {
        println!("Error in reading file.");
        -1
    }
}

fn main() {
    calc_ans("./input.txt");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ans() {
        assert_eq!(calc_ans("src/test.txt"), 143);
    }
}
