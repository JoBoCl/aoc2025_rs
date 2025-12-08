extern crate test;

use anyhow::Context;
use itertools::Itertools;
use solver::{Solver, SolverToAny};
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Reverse;

pub struct Day08 {
    points: Vec<Point>,
    diffs: BinaryHeap<Reverse<PointDiff>>,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone, Copy)]
struct Point {
    id: usize,
    x: u64,
    y: u64,
    z: u64,
}

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
struct PointDiff {
    diff: u64,
    l: Point,
    r: Point,
}

static ID: std::sync::atomic::AtomicUsize = 0.into();

impl TryFrom<String> for Point {
    type Error = anyhow::Error;

    fn try_from(value: String) -> anyhow::Result<Point> {
        let [sx, sy, sz] = value.splitn(3, ',').collect::<Vec<_>>()[..] else {
            anyhow::bail! {"could not split {value}"}
        };

        let x = sx
            .parse::<u64>()
            .context(format! {"could not parse {sx}"})?;
        let y = sy
            .parse::<u64>()
            .context(format! {"could not parse {sy}"})?;
        let z = sz
            .parse::<u64>()
            .context(format! {"could not parse {sz}"})?;

        Ok(Point { x, y, z, id: ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed) })
    }
}

impl Point {
    fn diff(&self, other: &Self) -> u64 {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y) + self.z.abs_diff(other.z)
    }
}

impl SolverToAny for Day08 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day08 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        let points: Vec<Point> = input.map(Point::try_from).try_collect()?;
        let mut diffs = BinaryHeap::new();
        for l in &points {
            for r in &points {
                if l == r {
                    continue;
                }
                let diff: u64 = l.diff(r);
                diffs.push(Reverse(PointDiff{diff , l: *l, r: *r}))
            }
        }
        Ok(Box::new(Day08{ points, diffs}))
    }
}

const PART_ONE_EXAMPLE_LIMIT: usize = 10;
const PART_ONE_PUZZLE_LIMIT: usize = 1000;

struct DSU {
    id: usize,
    parent_id: usize,
}

impl DSU {
    fn new(id: usize) -> Self {
        DSU{id, parent_id: id}
    }
}

struct DSF {
    entries: HashMap<usize, DSU>,
}

impl DSF {
    fn new(limit: usize)  -> Self {
        let mut entries = HashMap::new();

        for i in 0_usize..limit {
            entries.insert(i, DSU::new(i));
        }
        DSF{entries}
    }
}

impl Solver for Day08 {
    fn part_one(&self) -> anyhow::Result<String> {
        let limit = if self.points.len() < 100 { PART_ONE_EXAMPLE_LIMIT } else { PART_ONE_PUZZLE_LIMIT };
        let mut dsf = DSF::new(ID.load(std::sync::atomic::Ordering::Relaxed));
        Err(anyhow::anyhow! {"Not Implemented yet"})
    }

    fn part_two(&self) -> anyhow::Result<String> {
        Err(anyhow::anyhow! {"Not Implemented yet"})
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;

    use super::*;
    use test::Bencher;

    #[test]
    fn it_works_on_the_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day08/example.input")
            .lines()
            .map(String::from);

        let solver = Day08::try_create(Box::new(input)).unwrap();
        assert! {solver.part_one().is_err()};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day08/example.input")
            .lines()
            .map(String::from);

        let solver = Day08::try_create(Box::new(input)).unwrap();
        assert! {solver.part_two().is_err()};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day08/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day08::try_create(Box::new(input)).unwrap();
        assert! {solver.part_one().is_err()};
        assert! {solver.part_two().is_err()};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day08/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day08::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day08/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day08::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day08/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day08::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
