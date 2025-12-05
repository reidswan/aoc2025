const RAW_INPUT: &'static str = include_str!("inputs/day04.txt");

fn parse_input(input: &str) -> Vec<Vec<bool>> {
    input
        .lines()
        .map(|line| line.chars().map(|c| c == '@').collect())
        .collect()
}

const ADJACENT_DELTAS: [(isize, isize); 8] = [
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

fn find_accessible(input: &Vec<Vec<bool>>) -> Vec<(usize, usize)> {
    let height = input.len() as isize;
    input
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            let width = row.len() as isize;
            row.iter()
                .enumerate()
                .filter_map(|(x, elem)| {
                    if *elem
                        && ADJACENT_DELTAS
                            .iter()
                            .filter(|(dx, dy)| {
                                let adjx = x as isize + dx;
                                let adjy = y as isize + dy;
                                adjx >= 0
                                    && adjx < width as isize
                                    && adjy >= 0
                                    && adjy < height as isize
                                    && input[adjy as usize][adjx as usize]
                            })
                            .count()
                            < 4
                    {
                        Some((x, y))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        })
        .collect()
}

fn part1(input: &Vec<Vec<bool>>) -> usize {
    find_accessible(input).len()
}

fn part2(input: &mut Vec<Vec<bool>>, acc: usize) -> usize {
    let accessible = find_accessible(input);
    let count = accessible.len();
    for (x, y) in accessible {
        input[y][x] = false;
    }

    if count == 0 {
        acc
    } else {
        part2(input, acc + count)
    }
}

pub fn solve() {
    let mut input = parse_input(RAW_INPUT);

    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&mut input, 0));
}
