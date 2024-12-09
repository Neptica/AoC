use std::char;
use std::collections::HashSet;
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

fn find_guard(layout: Vec<Vec<char>>) -> (usize, usize) {
    let guard = ['^', 'v', '>', '<'];
    for y in 0..layout.len() {
        for x in 0..layout[y].len() {
            if guard.contains(&layout[y][x]) {
                return (x, y);
            }
        }
    }
    (0, 0)
}

fn part_one(
    x: usize,
    y: usize,
    direction: i32,
    map: &[Vec<char>],
) -> (Vec<(i32, i32)>, (usize, usize), bool) {
    let mut locations: Vec<(i32, i32)> = Vec::new();
    let mut x_last: usize = x;
    let mut y_last: usize = y;
    let mut go_on = true;

    let guard_type = ['^', '>', 'v', '<'];
    match guard_type[direction as usize] {
        '^' => {
            for y_move in (0..=y).rev() {
                if map[y_move][x] != '#' {
                    locations.push((x as i32, y_move as i32));
                    y_last = y_move;
                    if y_move == map.len() - 1 || y_move == 0 {
                        go_on = false
                    }
                } else {
                    break;
                }
            }
        }
        '>' => {
            for x_move in x..map[y].len() {
                if map[y][x_move] != '#' {
                    locations.push((x_move as i32, y as i32));
                    x_last = x_move;
                    if x_move == map[y].len() - 1 || x_move == 0 {
                        go_on = false
                    }
                } else {
                    break;
                }
            }
        }
        'v' => {
            for y_move in y..map.len() {
                if map[y_move][x] != '#' {
                    locations.push((x as i32, y_move as i32));
                    y_last = y_move;
                    if y_move == map.len() - 1 || y_move == 0 {
                        go_on = false
                    }
                } else {
                    break;
                }
            }
        }
        '<' => {
            for x_move in (0..=x).rev() {
                if map[y][x_move] != '#' {
                    locations.push((x_move as i32, y as i32));
                    x_last = x_move;
                    if x_move == map[y].len() - 1 || x_move == 0 {
                        go_on = false
                    }
                } else {
                    break;
                }
            }
        }
        _ => println!("Guard not actually found"),
    }
    (locations, (x_last, y_last), go_on)
}

fn check_loop(
    mut x: usize,
    mut y: usize,
    mut direction: i32,
    mut map: Vec<Vec<char>>,
) -> (i32, i32) {
    let mut obstacle_x = x;
    let mut obstacle_y = y;

    match direction {
        0 => obstacle_y -= 1,
        1 => obstacle_x += 1,
        2 => obstacle_y += 1,
        3 => obstacle_x -= 1,
        _ => println!("Invalid direction in check_loop"),
    }
    let obstacle = (obstacle_x as i32, obstacle_y as i32);
    map[obstacle_y][obstacle_x] = '#';
    let mut turn_points = vec![(x, y, direction)];
    direction = (direction + 1) % 4;
    let mut current_x = x;
    let mut current_y = y;

    let mut inbounds = true;
    let guard_type = ['^', '>', 'v', '<'];
    loop {
        x = current_x;
        y = current_y;
        match guard_type[direction as usize] {
            '^' => {
                for y_move in (0..=y).rev() {
                    if map[y_move][x] != '#' {
                        current_y = y_move;
                        if y_move == map.len() - 1 || y_move == 0 {
                            inbounds = false
                        }
                    } else {
                        break;
                    }
                }
            }
            '>' => {
                for x_move in x..map[0].len() {
                    if map[y][x_move] != '#' {
                        current_x = x_move;
                        if x_move == map[y].len() - 1 || x_move == 0 {
                            inbounds = false
                        }
                    } else {
                        break;
                    }
                }
            }
            'v' => {
                for y_move in y..map.len() {
                    if map[y_move][x] != '#' {
                        current_y = y_move;
                        if y_move == map.len() - 1 || y_move == 0 {
                            inbounds = false
                        }
                    } else {
                        break;
                    }
                }
            }
            '<' => {
                for x_move in (0..=x).rev() {
                    if map[y][x_move] != '#' {
                        current_x = x_move;
                        if x_move == map[y].len() - 1 || x_move == 0 {
                            inbounds = false
                        }
                    } else {
                        break;
                    }
                }
            }
            _ => println!("Guard not actually found"),
        }
        if !inbounds {
            return (-1, -1); // remove this from final HashSet
        }

        // don't check previous value because gotta walk away first
        for starting_pos in &turn_points[..turn_points.len() - 1] {
            if (current_x, current_y, direction) == *starting_pos {
                println!("Obstacle Location: {:?}", obstacle);
                return obstacle;
            }
        }

        // update after check
        turn_points.push((current_x, current_y, direction));
        direction = (direction + 1) % 4;
    }
}

fn calc(filename: &str) -> i32 {
    // part one
    if let Ok(map) = read_lines(filename) {
        let mut direction = 0;

        let layout: Vec<Vec<char>> = map
            .map_while(Result::ok)
            .map(|line| line.chars().collect())
            .collect();
        let (mut x, mut y) = find_guard(layout.clone());

        let guard = layout[y][x];
        match guard {
            '^' => direction = 0,
            '>' => direction = 1,
            'v' => direction = 2,
            '<' => direction = 3,
            _ => println!("Guard not actually found"),
        }

        let mut traveled: HashSet<(i32, i32)> = HashSet::new();
        let mut obstacle_placements: HashSet<(i32, i32)> = HashSet::new();
        let mut first_walk = true;
        loop {
            let walk = part_one(x, y, direction, &layout[..]);
            if first_walk {
                direction = (direction + 1) % 4;
                x = walk.1 .0;
                y = walk.1 .1;
                first_walk = false;
                if !walk.2 {
                    break;
                }
                continue;
            }
            for tuple in &walk.0[..walk.0.len() - 1] {
                traveled.insert(*tuple);
                let (x_check, y_check) = tuple;
                // println!("Here");
                match direction {
                    0 => {
                        if *y_check != 0 {
                            let (xer, yer) = check_loop(
                                *x_check as usize,
                                *y_check as usize,
                                direction,
                                layout.clone(),
                            );
                            if !traveled.contains(&(xer, yer)) {
                                obstacle_placements.insert((xer, yer));
                            }
                        }
                    }
                    1 => {
                        if *x_check as usize != layout[*y_check as usize].len() - 1 {
                            let (xer, yer) = check_loop(
                                *x_check as usize,
                                *y_check as usize,
                                direction,
                                layout.clone(),
                            );
                            if !traveled.contains(&(xer, yer)) {
                                obstacle_placements.insert((xer, yer));
                            }
                        }
                    }
                    2 => {
                        if *y_check as usize != layout.len() - 1 {
                            let (xer, yer) = check_loop(
                                *x_check as usize,
                                *y_check as usize,
                                direction,
                                layout.clone(),
                            );
                            if !traveled.contains(&(xer, yer)) {
                                obstacle_placements.insert((xer, yer));
                            }
                        }
                    }
                    3 => {
                        if *x_check != 0 {
                            let (xer, yer) = check_loop(
                                *x_check as usize,
                                *y_check as usize,
                                direction,
                                layout.clone(),
                            );
                            if !traveled.contains(&(xer, yer)) {
                                obstacle_placements.insert((xer, yer));
                            }
                        }
                    }
                    _ => println!("Guard not actually found"),
                }
            }
            if !walk.2 {
                break;
            }
            direction = (direction + 1) % 4;
            x = walk.1 .0;
            y = walk.1 .1;
        }
        let answer = traveled.len();
        println!("Squares Traveled: {}", answer);

        let mut ans2 = obstacle_placements.len();
        if obstacle_placements.contains(&(-1, -1)) {
            ans2 -= 1;
        }
        return ans2 as i32;
    }
    0
}

fn main() {
    let ans = calc("./input.txt");
    println!("Possible Loop Placements: {}", ans);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_main() {
        assert_eq!(calc("src/test.txt"), 6);
    }
}
