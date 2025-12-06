const RAW_INPUT: &'static str = include_str!("inputs/day06.txt");

#[derive(Clone, Copy, Debug)]
enum Operation {
    Plus,
    Times,
}

#[derive(Debug)]
struct Problem {
    elements: Vec<u128>,
    operation: Operation,
}

fn parse_input_p1(input: &str) -> Vec<Problem> {
    let lines = input
        .lines()
        .map(|line| line.split_ascii_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut problems = vec![];
    for i in 0..lines[0].len() {
        let elements = lines[0..lines.len() - 1]
            .iter()
            .map(|line| line[i].parse().unwrap())
            .collect();

        let operation = parse_operation(lines.last().unwrap()[i]);

        problems.push({
            Problem {
                elements,
                operation,
            }
        })
    }

    problems
}

fn parse_operation(s: &str) -> Operation {
    match s {
        "+" => Operation::Plus,
        "*" => Operation::Times,
        c => panic!("Unrecognized operation: {}", c),
    }
}

fn parse_input_p2(input: &str) -> Vec<Problem> {
    let lines = input.lines().collect::<Vec<_>>();
    let ops_line = lines
        .last()
        .unwrap()
        .split_ascii_whitespace()
        .collect::<Vec<_>>();

    let mut transposed = vec![String::with_capacity(lines.len()); lines[0].len()];

    for line in &lines[..lines.len() - 1] {
        for (x, ch) in line.chars().enumerate() {
            transposed[x].push(ch);
        }
    }

    let full = transposed.iter().fold(String::new(), |mut acc, curr| {
        acc.push_str(curr.trim());
        acc.push('\n');
        acc
    });

    let mut problems = vec![];
    for (i, section) in full.split("\n\n").enumerate() {
        let elements = section
            .lines()
            .map(|line| line.trim().parse().unwrap())
            .collect();
        let operation = parse_operation(ops_line[i]);

        problems.push(Problem {
            elements,
            operation,
        })
    }

    problems
}

fn solve_problems(input: &[Problem]) -> u128 {
    input
        .iter()
        .map(|problem| match problem.operation {
            Operation::Times => problem.elements.iter().fold(1, |acc, curr| acc * curr),
            Operation::Plus => problem.elements.iter().sum(),
        })
        .sum()
}

pub fn solve() {
    println!("Part 1: {}", solve_problems(&parse_input_p1(RAW_INPUT)));
    println!("Part 2: {}", solve_problems(&parse_input_p2(RAW_INPUT)));
}
