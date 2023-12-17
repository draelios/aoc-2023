use itertools::Itertools;
use std::{fs, collections::{HashMap, HashSet}};

pub fn run() {
    let input = fs::read_to_string("./inputs/input3.txt").unwrap();
    println!("Solution part 1: {}", solve_part1(&input));
    println!("Solution part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> i32 {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut sum = 0;
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let symbols = vec!['*', '$', '+', '#', '@', '&', '%', '!', '-', '/', '='];

    for (i, row) in grid.iter().enumerate() {
        let mut number = 0;
        let mut added = false;
        for (j, &cell) in row.iter().enumerate() {
            if !cell.is_digit(10) {
                number = 0;
                added = false;
                continue;
            }

            if number == 0 {    
                // Build the whole number once we find a digit & we don't have a number yet
                let mut k = j;
                while k < row.len() && row[k].is_digit(10) {
                    number = number * 10 + row[k].to_digit(10).unwrap() as i32;
                    k += 1;
                }
            }

            // Check if the number is valid
            for &(dx, dy) in &directions {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < row.len() as i32 {
                    match (symbols.iter().position(|a|a == &grid[nx as usize][ny as usize]), added){
                        (Some(_), false) => {
                            sum += number;
                            added = true;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    sum
}

fn solve_part2(input: &str) -> i32 {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let directions = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];
    let mut map: HashMap<(i32, i32), (i32, bool)> = HashMap::new();

    for (i, row) in grid.iter().enumerate() {
        let mut number = 0;
        let mut added = false;
        for (j, &cell) in row.iter().enumerate() {
            if !cell.is_digit(10) {
                number = 0;
                added = false;
                continue;
            }

            if number == 0 {    
                // Build the whole number once we find a digit & we don't have a number yet
                let mut k = j;
                while k < row.len() && row[k].is_digit(10) {
                    number = number * 10 + row[k].to_digit(10).unwrap() as i32;
                    k += 1;
                }
            }

            // Add to posible gear list
            for &(dx, dy) in &directions {
                let nx = i as i32 + dx;
                let ny = j as i32 + dy;
                if nx >= 0 && nx < grid.len() as i32 && ny >= 0 && ny < row.len() as i32 {
                    match ('*' == grid[nx as usize][ny as usize], added){
                        (true, false) => {
                            map.entry((nx,ny))
                                .and_modify(|gear: &mut (i32, bool)| {
                                    gear.0 *= number;
                                    gear.1 = true;
                                })
                                .or_insert((number, false));
                            added = true;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
    }
    map.iter().filter(|a| a.1.1 == true).map(|a| a.1.0).sum()
}


#[test]
fn test_part_1() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(solve_part1(input), 4361)
}

#[test]
fn test_part_2() {
    let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
    assert_eq!(solve_part2(input), 467835)
}

#[test]
fn test_part_2_1() {
    let input = "467..114..
...*......
..35..633.";
    assert_eq!(solve_part2(input), 16345)
}
