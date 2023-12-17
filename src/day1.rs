use itertools::Itertools;
use std::fs;

pub fn run() {
    let input = fs::read_to_string("./inputs/input1.txt").unwrap();
    println!("Solution part 1: {}", solve_part1(&input));
    println!("Solution part 2: {}", solve_part2(&input));
}

fn solve_part1(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line: &str| {
            let numbers: Vec<u32> = line
                .chars()
                .filter(|a| a.is_numeric())
                .map(|s| s.to_digit(10).unwrap())
                .collect();
            return match numbers.len() {
                0 => 0,
                _ => *numbers.first().unwrap() * 10 + *numbers.last().unwrap(),
            };
        })
        .sum()
}

fn solve_part2(input: &str) -> u32 {
    input
        .split("\n")
        .map(|line: &str| {
            let numbers: Vec<u32> = find_numbers(line);
            return match numbers.len() {
                0 => 0,
                _ => *numbers.first().unwrap() * 10 + *numbers.last().unwrap(),
            };
        })
        .sum()
}

fn find_numbers(line: &str) -> Vec<u32> {
    let number_words = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut result = vec![];

    for (letters, numbers) in number_words {
        line.match_indices(letters)
            .for_each(|(index, _)| result.push((index, numbers)));
        line.match_indices(numbers)
            .for_each(|(index, _)| result.push((index, numbers)));
    }

    result
        .into_iter()
        .sorted_by_key(|x| x.0)
        .map(|(_, value)| value.parse().unwrap())
        .collect()
}

#[test]
fn test_part_1() {
    let input = "1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet";
    assert_eq!(solve_part1(input), 142)
}

#[test]
fn test_part_2() {
    let input = "two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen";
    assert_eq!(solve_part2(input), 281)
}

#[test]
fn test_part_2_repeated_numbers() {
    let input = "vf19fourddfsvmzeight9
    mmg6fivegcthdonesix1eight
    7rzzdknxtbxdchsdfrkfivenjtbrjj
    2sdzxhxp
    vzvkjvngfjsxmponep9jppnqnbt8jtwo
    85fourfivetwo6xvhfxone9
    fivecgtwotwo3oneighth";
    assert_eq!(solve_part2(input), 343)
}
