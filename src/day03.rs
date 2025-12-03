extern crate test;

use solver::{Solver, SolverToAny};

pub struct Day03 {
    joltages: Vec<Vec<u8>>,
}

impl SolverToAny for Day03 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day03 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        Ok(Box::new(Day03 {
            joltages: input
                .map(|line| {
                    line.chars()
                        .into_iter()
                        .map(|c| c as u8 - '0' as u8)
                        .collect::<Vec<u8>>()
                })
                .collect::<Vec<_>>(),
        }))
    }

    fn joltages(count: usize, batteries: &[u8]) -> Vec<u8> {
        if count == 0 {
            return Vec::new();
        }
        let (idx, joltage) = batteries
                .iter()
                .enumerate()
                .rev()
                .skip(count - 1)
                .max_by(|l, r| l.1.cmp(r.1))
                .unwrap();
        let js = Day03::joltages(count - 1, &batteries[idx+1..]);

        return vec![vec![*joltage], js].concat();
    }

    fn joltage(batteries: Vec<u8>) -> u64 {
        batteries.iter().fold(0, |acc,elem| acc * 10 + *elem as u64)
    }
}

impl Solver for Day03 {
    fn part_one(&self) -> anyhow::Result<String> {
        Ok(self.joltages.iter().map(|b| Day03::joltages(2, b)).map(Day03::joltage).sum::<u64>().to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        Ok(self.joltages.iter().map(|b| Day03::joltages(12, b)).map(Day03::joltage).sum::<u64>().to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day03/example.input")
            .lines()
            .map(String::from);

        let solver = Day03::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "357"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day03/example.input")
            .lines()
            .map(String::from);

        let solver = Day03::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "3121910778619"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day03/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day03::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "17430"};
        assert_eq! {solver.part_two()?, "171975854269367"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day03/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day03::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day03/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day03::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day03/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day03::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
