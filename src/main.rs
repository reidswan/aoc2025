use std::{env::args, time::Instant};

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn run_day(n: usize) {
    println!("Day {}", n);
    let start = Instant::now();

    match n {
        1 => day01::solve(),
        2 => day02::solve(),
        3 => day03::solve(),
        4 => day04::solve(),
        5 => day05::solve(),
        6 => day06::solve(),
        _ => panic!("Not yet implemented: day {}", n),
    }

    let duration = Instant::now() - start;
    println!("Took {}ms", duration.as_millis());
}

const LATEST_DAY: usize = 6;

fn main() {
    run_day(
        args()
            .nth(1)
            .and_then(|s| s.parse::<usize>().ok())
            .unwrap_or(LATEST_DAY),
    );
}
