extern crate test;

use anyhow::Context;
use itertools::Itertools;
use solver::{Solver, SolverToAny};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

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

#[derive(Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Clone)]
struct PointDiff {
    diff: u64,
    l: Point,
    r: Point,
}

impl TryFrom<(String, usize)> for Point {
    type Error = anyhow::Error;

    fn try_from((value, id): (String, usize)) -> anyhow::Result<Point> {
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

        Ok(Point { x, y, z, id })
    }
}

impl Point {
    fn diff(&self, other: &Self) -> u64 {
        // technically should sqrt, but not necessary for comparisons
        // sqrt(x) < sqrt(y) iff x < y
        self.x.abs_diff(other.x).pow(2)
            + self.y.abs_diff(other.y).pow(2)
            + self.z.abs_diff(other.z).pow(2)
    }
}

impl SolverToAny for Day08 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day08 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        let mut points = Vec::new();
        let mut id = 0_usize;
        for line in input {
            if line.is_empty() {
                continue;
            }
            let point = Point::try_from((line, id))?;
            id += 1;
            points.push(point);
        }

        let mut diffs = BinaryHeap::new();
        for i in 0..points.len() {
            for j in (i + 1)..points.len() {
                let l = &points[i];
                let r = &points[j];
                let diff: u64 = l.diff(r);
                diffs.push(Reverse(PointDiff { diff, l: *l, r: *r }))
            }
        }

        Ok(Box::new(Day08 { points, diffs }))
    }
}

const PART_ONE_EXAMPLE_LIMIT: usize = 10;
const PART_ONE_PUZZLE_LIMIT: usize = 1000;

struct DSU {
    id: usize,
    parent_id: usize,
    size: usize,
}

impl DSU {
    fn new(id: usize) -> Self {
        DSU { id, parent_id: id, size: 1 }
    }

    fn set_parent(&mut self, parent: usize) {
        print!{"node {} parent {} -> {} --", self.id, self.parent_id, parent};
        self.parent_id = parent;
    }
    
    fn set_size(&mut self, size: usize) {
        self.size = size;
    }
}

struct DSF {
    entries: HashMap<usize, DSU>,
}

impl DSF {
    fn new(limit: usize) -> Self {
        let mut entries = HashMap::new();

        for i in 0_usize..limit {
            entries.insert(i, DSU::new(i));
        }
        DSF { entries }
    }

    fn merge(&mut self, l: usize, r: usize) -> bool {
        print!{"starting merge of {l}, {r}: "};
        let mut lp = self.parent(l);
        let mut rp = self.parent(r);
        
        // Already equal, nothing to do.
        print!{"l({lp}) ?= r({rp}) -- "};
        if lp == rp {
            println!{"done"};
            return false;
        }
        let (sl, sr) = (self.size(&lp) , self.size(&rp));

        print!{"|l|({sl}) <=> |r|({sr}) -- "};
        if self.size(&lp) > self.size(&rp) {
            (lp, rp) = (rp, lp);
        }

        let new_size = self.entries[&lp].size + self.entries[&rp].size;
        if let Some(n) = self.entries.get_mut(&lp) {
            n.set_parent(rp)
        } else {
            panic! {"could not find {lp}"}
        };
        self.entries.get_mut(&rp).unwrap().set_size(new_size);
        println!{"done"};
        true
    }

    fn parent(&self, mut id: usize) -> usize {
        let orig_id = id;
        while let Some(n) = self.entries.get(&id) {
            if n.parent_id == n.id {
                // println!{"{orig_id} parent = {}", n.id};
                return n.id;
            }
            id = n.parent_id;
        }
        panic! {"could not find parent for {orig_id}"}
    }
    
    fn size(&self, id: &usize) -> usize {
        self.entries[id].size
    }
}

impl Solver for Day08 {
    fn part_one(&self) -> anyhow::Result<String> {
        let limit = if self.points.len() < 100 {
            PART_ONE_EXAMPLE_LIMIT
        } else {
            PART_ONE_PUZZLE_LIMIT
        };
        let mut dsf = DSF::new(self.points.len());
        let mut connections = self.diffs.clone();
        let mut i = 0;
        while i < limit {
            if let Some(connection) = connections.pop() {
                let (lid, rid) = (connection.0.l.id, connection.0.r.id);
                if dsf.merge(lid, rid) {
                    i += 1;
                }
            } else {
                panic! {"not enough pairs!"}
            }
        }
        let mut parents_by_size = HashMap::new();
        for point in 0..self.points.len() {
            let parent = dsf.parent(point);
            let size = dsf.size(&parent);
            parents_by_size.insert(parent, size);
        }
        // println!{"parents by size: {:?}", parents_by_size.iter().sorted_by_key(|e| e.1).rev().collect_vec()};
        Ok(parents_by_size
            .into_values()
            .sorted()
            .rev()
            .take(3)
            .product::<usize>()
            .to_string())
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
        assert_eq! {solver.part_one()?, "40"};
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
        assert_eq! {solver.part_one()?, ""};
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
