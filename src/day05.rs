const RAW_INPUT: &'static str = include_str!("inputs/day05.txt");

struct Input {
    ranges: Vec<(usize, usize)>,
    ingredients: Vec<usize>,
}

fn parse_input(input: &str) -> Input {
    let (ranges_str, ingredients_str) = input.split_once("\n\n").unwrap();

    let mut ranges = ranges_str
        .lines()
        .map(|line| {
            let (s, e) = line.split_once('-').unwrap();
            (s.parse().unwrap(), e.parse().unwrap())
        })
        .collect::<Vec<(usize, usize)>>();

    ranges.sort_by(|(start1, _), (start2, _)| start1.cmp(start2));

    let mut ingredients: Vec<usize> = ingredients_str
        .lines()
        .map(|line| line.trim().parse().unwrap())
        .collect();

    ingredients.sort();

    Input {
        ranges: collapse_ranges(&ranges),
        ingredients,
    }
}

fn part1(input: &Input) -> usize {
    input
        .ingredients
        .iter()
        .filter(|&ingredient| {
            input
                .ranges
                .iter()
                .any(|(start, end)| start <= ingredient && ingredient <= end)
        })
        .count()
}

fn collapse_ranges(raw_ranges: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut ranges = vec![];

    let mut curr_range = raw_ranges[0];

    for &(start, end) in &raw_ranges[1..] {
        if curr_range.0 <= start && start <= curr_range.1 {
            curr_range = (curr_range.0, usize::max(end, curr_range.1));
        } else {
            ranges.push(curr_range);
            curr_range = (start, end);
        }
    }

    ranges.push(curr_range);

    ranges
}

fn part2(input: &Input) -> usize {
    input.ranges.iter().map(|&(a, b)| b - a + 1).sum()
}

pub fn solve() {
    let input = parse_input(RAW_INPUT);
    println!("Part 1: {}", part1(&input));
    println!("Part 2: {}", part2(&input));
}
