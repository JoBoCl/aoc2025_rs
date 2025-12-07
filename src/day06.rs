extern crate test;

use anyhow::Context;
use itertools::Itertools;
use solver::{Solver, SolverToAny};
use std::fmt::{Display, Formatter};

pub struct Day06 {
    input: Vec<String>,
}

struct Question {
    numbers: Vec<u64>,
    op: Op,
}

impl Question {
    fn answer(&self) -> u64 {
        let op = match self.op {
            Op::Mul => |l, r| l * r,
            Op::Add => |l, r| l + r,
        };
        let unit = match self.op {
            Op::Mul => 1_u64,
            Op::Add => 0_u64,
        };
        self.numbers.iter().fold(unit, op)
    }
}

impl Display for Question {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{:?} {:?} = {}", self.numbers, self.op, self.answer()}
    }
}

#[derive(Debug)]
enum Op {
    Mul,
    Add,
}

impl SolverToAny for Day06 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day06 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        Ok(Box::new(Day06 {
            input: input.filter(|s| !s.is_empty()).collect_vec(),
        }))
    }
}

impl Solver for Day06 {
    fn part_one(&self) -> anyhow::Result<String> {
        let mut numbers = Vec::new();
        let mut ops = Vec::new();

        for line in &self.input {
            let mut ns = Vec::new();
            for entry in line.split_ascii_whitespace() {
                match entry {
                    "*" => ops.push(Op::Mul),
                    "+" => ops.push(Op::Add),
                    s => {
                        ns.push(s.parse::<u64>().context(format! {"could not parse {s}"})?);
                    }
                }
            }
            if !ns.is_empty() {
                numbers.push(ns);
            }
        }

        let mut questions = Vec::new();
        for (idx, op) in ops.into_iter().enumerate() {
            let mut ns = Vec::new();
            for item in &numbers {
                ns.push(item[idx]);
            }
            questions.push(Question { op, numbers: ns });
        }

        Ok(questions
            .iter()
            .map(Question::answer)
            .sum::<u64>()
            .to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        let mut questions = Vec::new();

        let n = self.input[0].len();
        let chars = self
            .input
            .iter()
            .map(|l| (l.bytes().collect_vec(), l.starts_with(['*', '+'])))
            .collect_vec();
        let mut numbers = Vec::new();
        let mut op = Op::Mul;

        let mut number = 0_u64;
        for i in 0..n {
            if chars.iter().all(|c| c.0[i].is_ascii_whitespace()) {
                questions.push(Question { op, numbers });
                numbers = Vec::new();
                op = Op::Mul;
                continue;
            }
            for (line, is_op) in &chars {
                if !is_op {
                    let c = line[i];
                    match c {
                        b'0'..=b'9' => {
                            number = (number * 10) + (c - b'0') as u64;
                        }
                        b' ' => {}
                        c => panic! {"did not expect {c}"},
                    }
                } else {
                    match line[i] {
                        b'*' => op = Op::Mul,
                        b'+' => op = Op::Add,
                        b' ' => {}
                        c => panic! {"did not expect {c:?}"},
                    }
                }
            }
            numbers.push(number);
            number = 0;
        }

        questions.push(Question { op, numbers });

        Ok(questions
            .iter()
            .map(Question::answer)
            .sum::<u64>()
            .to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day06/example.input")
            .lines()
            .map(String::from);

        let solver = Day06::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "4277556"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day06/example.input")
            .lines()
            .map(String::from);

        let solver = Day06::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "3263827"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day06/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day06::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "5873191732773"};
        assert_eq! {solver.part_two()?, "11386445308378"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day06/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day06::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day06/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day06::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day06/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day06::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
