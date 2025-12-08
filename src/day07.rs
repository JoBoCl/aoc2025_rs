extern crate test;

use anyhow::bail;
use itertools::Itertools;
use solver::{Solver, SolverToAny};
use std::collections::{BTreeSet, HashMap, HashSet};
use std::fmt::{Display, Formatter};

pub struct Day07 {
    map: Vec<Vec<Space>>,
}

#[derive(Debug, PartialEq, Eq)]
enum Space {
    Empty,
    Splitter,
    Source,
}

impl TryFrom<char> for Space {
    type Error = anyhow::Error;

    fn try_from(c: char) -> anyhow::Result<Self> {
        match c {
            'S' => Ok(Space::Source),
            '^' => Ok(Space::Splitter),
            '.' => Ok(Space::Empty),
            _ => bail! {"could not match {c:?}"},
        }
    }
}

impl Display for Space {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Space::Empty => '.',
            Space::Splitter => '^',
            Space::Source => 'S',
        };
        write! {f, "{}", c}
    }
}

impl SolverToAny for Day07 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day07 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        Ok(Box::new(Day07 {
            map: input
                .map(|line| line.chars().map(Space::try_from).try_collect())
                .try_collect()?,
        }))
    }
}

impl Solver for Day07 {
    fn part_one(&self) -> anyhow::Result<String> {
        let mut beams = HashSet::new();
        let Some(x) = self.map[0].iter().position(|s| *s == Space::Source) else {
            panic! {"first line did not have start point"}
        };
        beams.insert(x);

        let mut splits = 0;
        let mut empty_only = false;
        for row in self.map.iter().skip(1) {
            empty_only = !empty_only;
            let mut new_beams = HashSet::new();
            for beam in beams {
                match row.get(beam) {
                    Some(Space::Source) => panic! {"should only have a single source"},
                    Some(Space::Splitter) => {
                        splits += 1;
                        new_beams.insert(beam - 1);
                        new_beams.insert(beam + 1);
                    }
                    Some(Space::Empty) => {
                        new_beams.insert(beam);
                    }
                    None => {}
                }
            }
            beams = new_beams;
        }

        Ok(splits.to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        let mut routes_to: HashMap<(usize, usize), usize> = HashMap::new();

        let mut beams = BTreeSet::new();
        let Some(x) = self.map[0].iter().position(|s| *s == Space::Source) else {
            panic! {"first line did not have start point"}
        };
        beams.insert(x);
        routes_to.insert((x, 0_usize), 1);

        for (y, row) in self.map.iter().enumerate().skip(1) {
            let mut new_beams = BTreeSet::new();
            for beam in beams {
                match row.get(beam) {
                    Some(Space::Source) => panic! {"should only have a single source"},
                    Some(Space::Splitter) => {
                        let above = routes_to[&(beam, y - 1)];
                        if !new_beams.insert(beam - 1) {
                            let left = routes_to[&(beam - 1, y)];
                            routes_to.insert((beam - 1, y), left + above);
                        } else {
                            routes_to.insert((beam - 1, y), above);
                        }
                        new_beams.insert(beam + 1);
                        routes_to.insert((beam + 1, y), above);
                    }
                    Some(Space::Empty) => {
                        new_beams.insert(beam);
                        let prior = routes_to.get(&(beam, y)).unwrap_or(&0);
                        routes_to.insert((beam, y), prior + routes_to[&(beam, y - 1)]);
                    }
                    None => {
                        routes_to.insert((beam, y), routes_to[&(beam, y - 1)]);
                    }
                }
            }
            beams = new_beams;
        }

        let mut total = 0;
        for ((_, y), n) in routes_to {
            if y == self.map.len() - 1 {
                total += n;
            }
        }

        Ok(total.to_string())
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day07/example.input")
            .lines()
            .map(String::from);

        let solver = Day07::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "21"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day07/example.input")
            .lines()
            .map(String::from);

        let solver = Day07::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "40"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day07/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day07::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "1592"};
        assert_eq! {solver.part_two()?, "17921968177009"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day07/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day07::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day07/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day07::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day07/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day07::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
