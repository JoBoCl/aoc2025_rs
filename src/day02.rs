extern crate test;

use std::ops::RangeInclusive;

use itertools::Itertools;
use solver::{Solver, SolverToAny};

pub struct Day02 {
    ranges: Vec<RangeInclusive<u64>>,
}

impl SolverToAny for Day02 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day02 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        let mut ranges = Vec::new();
        for line in input {
            for group in line.split(',') {
                if let Some((l, r)) = group.split_once('-') {
                    let left = l.parse::<u64>();
                    let right = r.parse::<u64>();
                    match (left, right) {
                        (Ok(pl), Ok(pr)) => ranges.push(pl..=pr),
                        (l, r) => {
                            panic! {"could not parse {l:?} or {r:?}"}
                        }
                    }
                }
            }
        }

        Ok(Box::new(Day02 { ranges }))
    }

    // 10..100 => 11, 100..1000 => None, 1000..10000 => 101
    fn multiplier(number: &u64) -> Option<u64> {
        let length = number.ilog10(); // 10..100 => 1, 100..1000 => 2, 1000..10000 => 4
        if length % 2 == 0 {
            return None;
        }
        Some(10_u64.pow(1 + length / 2) + 1)
    }

    fn repeated_segment(number: &u64) -> bool {
        if let Some(mul) = Day02::multiplier(number) {
            number % mul == 0
        } else {
            false
        }
    }

    fn repeated_segments(number: &u64) -> bool {
        let s = number.to_string();
        let l = s.len();
        for i in 1..=(l / 2) {
            if l % i != 0 {
                continue;
            }
            let chunks = s.chars().chunks(i);
            if chunks
                .into_iter()
                .map(|c| c.collect::<String>())
                .all_equal()
            {
                return true;
            }
        }
        false
    }
}

impl Solver for Day02 {
    fn part_one(&self) -> anyhow::Result<String> {
        Ok(self
            .ranges
            .iter()
            .flat_map(|range| range.clone().into_iter().filter(Day02::repeated_segment))
            .sum::<u64>()
            .to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        Ok(self
            .ranges
            .iter()
            .flat_map(|range| range.clone().into_iter().filter(Day02::repeated_segments))
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
        let input = include_str!("../puzzles/day02/example.input")
            .lines()
            .map(String::from);

        let solver = Day02::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "1227775554"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day02/example.input")
            .lines()
            .map(String::from);

        let solver = Day02::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "4174379265"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day02/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day02::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "40398804950"};
        assert_eq! {solver.part_two()?, "65794984339"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day02/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day02::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day02/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day02::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day02/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day02::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
