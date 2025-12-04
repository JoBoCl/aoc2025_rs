extern crate test;

use itertools::Itertools;
use solver::{Solver, SolverToAny};

pub struct Day04 {
    map: Floorplan,
}

impl SolverToAny for Day04 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[derive(Debug, Clone)]
struct Floorplan {
    floorplan: Vec<Vec<bool>>,
}

impl Day04 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        let floorplan = input
            .map(|line| line.chars().map(|c| c == '@').collect_vec())
            .collect_vec();
        assert! {floorplan.iter().map(|l| l.len()).all_equal()};
        Ok(Box::new(Day04 {
            map: Floorplan { floorplan },
        }))
    }
}

impl Floorplan {
    fn to_remove(&self) -> Vec<(usize, usize)> {
        let mut to_remove = Vec::new();
        for (y, row) in self.floorplan.iter().enumerate() {
            for (x, paper) in row.iter().enumerate() {
                if *paper && self.adjacent(x, y) < 4 {
                    to_remove.push((x, y));
                }
            }
        }
        to_remove
    }

    fn adjacent(&self, x: usize, y: usize) -> usize {
        let mut sum = 0;
        for i in -1..=1 {
            for j in -1..=1 {
                if i == 0 && j == 0 {
                    continue;
                }
                let a = x
                    .checked_add_signed(i)
                    .filter(|a| *a < self.floorplan[0].len());
                let b = y
                    .checked_add_signed(j)
                    .filter(|b| *b < self.floorplan.len());
                match (a, b) {
                    (Some(a), Some(b)) if self.floorplan[b][a] => {
                        sum += 1;
                    }
                    _ => {}
                }
            }
        }
        sum
    }

    fn remove(&mut self, point: (usize, usize)) {
        self.floorplan[point.1][point.0] = false;
    }
}

impl Solver for Day04 {
    fn part_one(&self) -> anyhow::Result<String> {
        Ok(self.map.to_remove().len().to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        let mut removed = 0;
        let mut floorplan = self.map.clone();
        loop {
            let to_remove = floorplan.to_remove();

            if to_remove.is_empty() {
                break;
            }
            removed += to_remove.len();
            for point in to_remove {
                floorplan.remove(point);
            }
        }
        Ok(removed.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day04/example.input")
            .lines()
            .map(String::from);

        let solver = Day04::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "13"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day04/example.input")
            .lines()
            .map(String::from);

        let solver = Day04::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "43"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day04/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day04::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "1464"};
        assert_eq! {solver.part_two()?, "8409"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day04/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day04::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day04/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day04::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day04/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day04::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
