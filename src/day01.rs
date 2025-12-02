extern crate test;

use anyhow::Context;
use itertools::Itertools;

use solver::{Solver, SolverToAny};

pub struct Day01 {
    instructions: Vec<Instruction>,
    start: u16,
}

impl SolverToAny for Day01 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

enum Instruction {
    Left(u16),
    Right(u16),
}

impl TryFrom<String> for Instruction {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Instruction> {
        let step = value
            .trim_start_matches(|c| c == 'L' || c == 'R')
            .parse::<u16>()
            .with_context(|| format! {"could not parse {value}"})?;
        if value.starts_with('R') {
            return Ok(Instruction::Right(step));
        } else if value.starts_with('L') {
            return Ok(Instruction::Left(step));
        } else {
            anyhow::bail! {"{value} doesn't start with l or r!"}
        }
    }
}

impl Day01 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        Ok(Box::new(Day01 {
            instructions: input.map(Instruction::try_from).try_collect()?,
            start: 50,
        }))
    }
}

impl Solver for Day01 {
    fn part_one(&self) -> anyhow::Result<String> {
        let mut position = self.start;
        let mut zeroes = 0;
        for i in &self.instructions {
            match i {
                Instruction::Left(n) => {
                    position = (position + 100 - (n % 100)) % 100;
                }
                Instruction::Right(n) => {
                    position = (position + n) % 100;
                }
            }
            if position == 0 {
                zeroes += 1;
            }
        }
        Ok(zeroes.to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        let mut position = self.start;
        let mut zeroes: u16 = 0;
        for i in &self.instructions {
            match i {
                Instruction::Left(n) if *n < position => {
                    position -= n;
                }
                Instruction::Left(n) => { // n >= position
                    let mut pos = position;
                    let mut i = 0;
                    while i < *n {
                        pos = pos.checked_sub(1).unwrap_or_else(|| 99);
                        if pos == 0 {
                            zeroes += 1;
                        }
                        i += 1;
                    }
                    position = pos;
                }
                Instruction::Right(n) => {
                    let advance = position + n;
                    position = advance % 100;
                    zeroes += advance / 100;
                }
            }
        }
        Ok(zeroes.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day01/example.input")
            .lines()
            .map(String::from);

        let solver = Day01::try_create(Box::new(input)).unwrap();
        assert! {solver.part_one().is_ok_and(|s| s == "3")};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day01/example.input")
            .lines()
            .map(String::from);

        let solver = Day01::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "6"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day01/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day01::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "962"};
        assert_eq! {solver.part_two()?, "5782"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day01/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day01::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day01/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day01::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day01/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day01::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
