use std::collections::HashSet;

const RAW_INPUT: &'static str = include_str!("inputs/day02.txt");

fn parse_input(input: &str) -> Vec<(usize, usize)> {
    input
        .split(',')
        .map(|range| {
            let mut split = range.split('-');
            let start = split.next().unwrap().parse().unwrap();
            let end = split.next().unwrap().parse().unwrap();
            (start, end)
        })
        .collect()
}

fn count_digits(i: usize) -> u32 {
    return ((i as f64).log10() + 1.).floor() as u32;
}

fn find_bad_ids(start: usize, end: usize) -> usize {
    let start_len = u32::max(count_digits(start) / 2, 1);
    let end_len = count_digits(end) / 2 + 1;
    let mut sum = 0;

    for len in start_len..=end_len {
        let rng_start = 10usize.pow(len - 1);
        let rng_end = 10usize.pow(len);
        'inner: for elm in rng_start..rng_end {
            let exp = count_digits(elm);
            let full = elm + 10usize.pow(exp) * elm;
            if start <= full && full <= end {
                sum += full
            } else if full > end {
                break 'inner;
            }
        }
    }

    sum
}

fn part1(input: &[(usize, usize)]) -> usize {
    input.iter().map(|&(a, b)| find_bad_ids(a, b)).sum()
}

fn find_bad_ids_p2(start: usize, end: usize) -> usize {
    let start_len = 1;
    let end_len = count_digits(end) / 2 + 1;
    let mut found = HashSet::new();

    for len in start_len..=end_len {
        let rng_start = 10usize.pow(len - 1);
        let rng_end = 10usize.pow(len);
        'inner: for elm in rng_start..rng_end {
            let exp = count_digits(elm);
            let mut full = elm + 10usize.pow(exp) * elm;

            if full > end {
                break 'inner;
            }

            while full <= end {
                if start <= full && full <= end {
                    found.insert(full);
                }

                full = full * 10usize.pow(exp) + elm;
            }
        }
    }

    found.iter().sum()
}

fn part2(input: &[(usize, usize)]) -> usize {
    input.iter().map(|&(a, b)| find_bad_ids_p2(a, b)).sum()
}

pub fn solve() {
    let input = parse_input(RAW_INPUT);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
