const RAW_INPUT: &'static str = include_str!("inputs/day03.txt");

fn parse_input(input: &str) -> Vec<Vec<u8>> {
    input
        .lines()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.chars()
                .filter_map(|c| c.to_digit(10).map(|it| it as u8))
                .collect()
        })
        .collect()
}

fn highest_joltage(bank: &Vec<u8>, n_digits: usize) -> usize {
    // Construct a number by always taking the highest remaining digit,
    // while ensuring that there are at least as many digits left over
    // as there are remaining spots in the number we are constructing
    let mut joltage = 0;
    let mut curr_max_i = 0;
    let mut curr_max;
    for digit in 0..n_digits {
        curr_max = bank[curr_max_i];
        for i in curr_max_i..bank.len() - (n_digits - digit - 1) {
            if bank[i] > curr_max {
                curr_max = bank[i];
                curr_max_i = i
            }
        }
        joltage = joltage * 10 + curr_max as usize;
        curr_max_i += 1;
    }

    joltage
}

fn part1(banks: &Vec<Vec<u8>>) -> usize {
    banks.iter().map(|bank| highest_joltage(bank, 2)).sum()
}

fn part2(banks: &Vec<Vec<u8>>) -> usize {
    banks.iter().map(|bank| highest_joltage(bank, 12)).sum()
}

pub fn solve() {
    let input = parse_input(RAW_INPUT);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
