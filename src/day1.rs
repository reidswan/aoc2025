use std::ops::Neg;

const RAW_INPUT: &'static str = include_str!("inputs/day01.txt");

fn parse_input(input: &str) -> Vec<isize> {
    input
        .lines()
        .filter(|line| line.trim().len() > 0)
        .map(|line| {
            let (dir, amt_s) = line.trim().split_at(1);
            let amount = amt_s.parse::<isize>().unwrap();
            if dir == "L" {
                amount.neg()
            } else {
                amount
            }
        })
        .collect()
}

fn part1(input: &[isize]) -> isize {
    let (_, count) = input
        .iter()
        .fold((50isize, 0isize), |(loc, count), rotation| {
            let new_loc = (loc + rotation).rem_euclid(100);
            if new_loc == 0 {
                (new_loc, count + 1)
            } else {
                (new_loc, count)
            }
        });
    count
}

fn part2(input: &[isize]) -> isize {
    let (_, count) = input
        .iter()
        .fold((50isize, 0isize), |(loc, count), rotation| {
            let new_loc_raw = loc + rotation;
            let new_loc = new_loc_raw.rem_euclid(100);

            let delta = if new_loc_raw >= 100 {
                new_loc_raw / 100
            } else if new_loc_raw <= 0 {
                new_loc_raw.abs() / 100 + if loc == 0 { 0 } else { 1 }
            } else {
                0
            };

            (new_loc, count + delta)
        });
    count
}

pub fn solve() {
    println!("Part 1: {}", part1(&parse_input(RAW_INPUT)));
    println!("Part 2: {}", part2(&parse_input(RAW_INPUT)));
}
