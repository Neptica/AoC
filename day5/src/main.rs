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

fn rearrange(mut update: Vec<i32>, guides: Vec<i32>, rules: Vec<Vec<i32>>) -> Vec<i32> {
    let mut complete = false;
    let mut pos = 1;
    loop {
        let mut moved = false;
        for index in pos..update.len() {
            if let Ok(rule_index) = guides.binary_search(&update[index]) {
                for cindex in 0..index {
                    if rules[rule_index].contains(&update[cindex]) {
                        let current = update.remove(index);
                        update.insert(cindex, current);
                        pos = cindex + 1;
                        moved = true;
                        break;
                    }
                }
                if moved {
                    break;
                }
            }
            if index == update.len() - 1 {
                complete = true;
            }
        }
        if complete {
            break;
        }
    }
    update
}

fn part_one(file: &str) -> i32 {
    if let Ok(input) = read_lines(file) {
        let (guides, rules, lists) = create_rules(input);
        // for i in 0..guides.len() {
        //     println!("{}: ({}, {:?})", i, guides[i], rules[i]);
        // }
        let mut ans = 0;

        for list in lists {
            let mut ready = true;
            for index in 1..list.len() {
                if let Ok(rule_index) = guides.binary_search(&list[index]) {
                    for cindex in 0..index {
                        if rules[rule_index].contains(&list[cindex]) {
                            ready = false;
                        }
                    }
                }
            }
            if ready {
                ans += list[(list.len() - 1) / 2];
            }
        }

        ans
    } else {
        println!("Error in reading file.");
        -1
    }
}

fn part_two(file: &str) -> i32 {
    if let Ok(input) = read_lines(file) {
        let (guides, rules, lists) = create_rules(input);
        let mut ans = 0;

        for mut list in lists {
            let mut ready = true;
            for index in 1..list.len() {
                if let Ok(rule_index) = guides.binary_search(&list[index]) {
                    for cindex in 0..index {
                        if rules[rule_index].contains(&list[cindex]) {
                            ready = false;
                        }
                    }
                }
            }
            if !ready {
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
            }
        }

        ans
    } else {
        println!("Error in reading file.");
        -1
    }
}

fn main() {
    // part one
    let ans = part_one("./input.txt");
    println!("Sum of the Middle Element of Proper Lists: {}", ans);

    // part two
    let ans = part_two("./input.txt");
    println!("Sum of the Middle Element of Proper Lists: {}", ans);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ans() {
        assert_eq!(part_one("src/test.txt"), 143);
    }
}
