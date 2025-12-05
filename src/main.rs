#![feature(test)]

use clap::Parser;

// BEGIN_MOD_LIST
mod day00;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
// END_MOD_LIST

use solver::Solver;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: usize,
    #[arg(short, long)]
    input: String,
}

impl read::HasFile for Args {
    fn file(&self) -> String {
        self.input.clone()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (input, flags) = read::input_with_type::<Args>(None);
    let solver: Box<dyn Solver> = match flags.day {
        // BEGIN_SOLVER_LIST
        0 => day00::Day00::try_create(input),
        1 => day01::Day01::try_create(input),
        2 => day02::Day02::try_create(input),
        3 => day03::Day03::try_create(input),
        4 => day04::Day04::try_create(input),
        5 => day05::Day05::try_create(input),
        // END_SOLVER_LIST
        _ => panic! {"Failed to find solver"},
    }?;
    println! {"{}", solver.part_one()?};
    println! {"{}", solver.part_two()?};
    Ok(())
}
