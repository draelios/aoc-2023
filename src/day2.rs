use itertools::Itertools;
use std::{cmp, fs};

pub fn run() {
    let input = fs::read_to_string("./inputs/input2.txt").unwrap();
    println!("Solution part 1: {}", solve_part1(&input));
    println!("Solution part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut line: Vec<&str> = line.split(":").collect();
            let blocks: Vec<(u32, &str)> = line
                .pop()
                .unwrap()
                .split(|c| c == ',' || c == ';')
                .map(|a| {
                    let a: (&str, &str) = a.trim().split(' ').collect_tuple().unwrap();
                    (a.0.parse::<u32>().unwrap(), a.1)
                })
                .collect_vec();
            let game: u32 = line
                .pop()
                .unwrap()
                .strip_prefix("Game ")
                .unwrap()
                .parse()
                .unwrap();
            let mut allowed = true;
            for block in blocks {
                match (block.0, block.1) {
                    (a, "red") if a > 12 => {
                        allowed = false;
                        break;
                    }
                    (a, "green") if a > 13 => {
                        allowed = false;
                        break;
                    }
                    (a, "blue") if a > 14 => {
                        allowed = false;
                        break;
                    }
                    _ => {}
                };
            }

            if (allowed) {
                return game;
            } else {
                return 0;
            }
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let mut max_colors = vec![0; 3];
            let mut line: Vec<&str> = line.split(":").collect();
            let blocks: Vec<(u32, &str)> = line
                .pop()
                .unwrap()
                .split(|c| c == ',' || c == ';')
                .map(|a| {
                    let a: (&str, &str) = a.trim().split(' ').collect_tuple().unwrap();
                    (a.0.parse().unwrap(), a.1)
                })
                .collect_vec();

            for block in blocks {
                match (block.0, block.1) {
                    (a, "red") => max_colors[0] = cmp::max(max_colors[0], a),
                    (a, "green") => max_colors[1] = cmp::max(max_colors[1], a),
                    (a, "blue") => max_colors[2] = cmp::max(max_colors[2], a),
                    _ => {}
                };
            }
            
            max_colors[0] * max_colors[1] * max_colors[2]
        })
        .sum()
}

#[test]
fn test_part_1() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(solve_part1(input), 8)
}

#[test]
fn test_part_2() {
    let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
    assert_eq!(solve_part2(input), 2286)
}
