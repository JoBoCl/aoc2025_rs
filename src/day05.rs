extern crate test;

use anyhow::Context;
use solver::{Solver, SolverToAny};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::RangeInclusive;

pub struct Day05 {
    normalised_fresh_ingredients: HashSet<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl SolverToAny for Day05 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

trait RangeBound = PartialOrd + Ord + Clone + Copy + Hash + PartialEq + Eq + Display;

#[derive(Debug, Clone)]
struct MergedRanges<T: RangeBound> {
    range: RangeInclusive<T>,
    children: HashSet<RangeInclusive<T>>,
}

impl<T: RangeBound> PartialEq for MergedRanges<T> {
    fn eq(&self, other: &Self) -> bool {
        self.range == other.range
    }
}

impl<T: RangeBound> Display for MergedRanges<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write! {f, "{}..={}", self.range.start(), self.range.end()}
    }
}

impl<T: RangeBound> Eq for MergedRanges<T> {}

impl<T: RangeBound> Hash for MergedRanges<T> {
    fn hash<H>(&self, h: &mut H)
    where
        H: Hasher,
    {
        h.write(&self.to_string().into_bytes());
    }
}

impl<T: RangeBound> MergedRanges<T> {
    fn create(range: &RangeInclusive<T>) -> Self {
        MergedRanges {
            range: range.clone(),
            children: HashSet::from([range.clone()]),
        }
    }

    fn merge(&self, other: &Self) -> Option<Self> {
        if let Some(range) = Day05::overlap(&self.range, &other.range) {
            Some(MergedRanges {
                range,
                children: self.children.union(&other.children).cloned().collect(),
            })
        } else {
            None
        }
    }
}

impl Day05 {
    pub fn try_create(
        mut input: Box<dyn Iterator<Item = String>>,
    ) -> anyhow::Result<Box<dyn Solver>> {
        let mut fresh_ingredients = HashSet::new();
        let mut available_ingredients: Vec<u64> = Vec::new();
        for line in input.by_ref() {
            if line.is_empty() {
                break;
            }
            if let Some((start, end)) = line.split_once('-') {
                let parsed_start = start
                    .parse::<u64>()
                    .with_context(|| format! {"could not parse '{start}'"})?;
                let parsed_end = end
                    .parse::<u64>()
                    .with_context(|| format! {"could not parse '{end}'"})?;
                fresh_ingredients.insert(parsed_start..=parsed_end);
            } else {
                anyhow::bail! {"could not parse '{line}'"}
            }
        }

        let mut normalised_fresh_ingredients = fresh_ingredients
            .iter()
            .map(MergedRanges::create)
            .collect::<HashSet<_>>();
        loop {
            let mut new_normalised_fresh_ingredients = HashSet::new();
            let mut merged_away = HashSet::new();
            let mut merged = false;
            for r1 in &normalised_fresh_ingredients {
                for r2 in &normalised_fresh_ingredients {
                    if r1 == r2 {
                        continue;
                    }
                    if let Some(r) = r1.merge(r2) {
                        match (r == *r1, r == *r2) {
                            (true, true) => panic! {"should have already ignored matching ranges"},
                            (false, false) => {
                                merged_away.insert(r1.clone());
                                merged_away.insert(r2.clone());
                                merged |= new_normalised_fresh_ingredients.insert(r);
                            }
                            (true, false) => {
                                merged_away.insert(r2.clone());
                                merged |= new_normalised_fresh_ingredients.insert(r);
                            }
                            (false, true) => {
                                merged_away.insert(r1.clone());
                                merged |= new_normalised_fresh_ingredients.insert(r);
                            }
                        }
                    } else {
                        if !merged_away.contains(r1) {
                            new_normalised_fresh_ingredients.insert(r1.clone());
                        }
                        if !merged_away.contains(r2) {
                            new_normalised_fresh_ingredients.insert(r2.clone());
                        }
                    }
                }
            }
            normalised_fresh_ingredients = new_normalised_fresh_ingredients
                .into_iter()
                .filter(|nfi| !merged_away.contains(nfi))
                .collect();
            if !merged {
                break;
            }
        }

        for line in input {
            if line.is_empty() {
                break;
            }
            let ingredient = line
                .parse::<u64>()
                .with_context(|| format! {"could not parse '{line}'"})?;
            available_ingredients.push(ingredient);
        }
        Ok(Box::new(Day05 {
            normalised_fresh_ingredients: normalised_fresh_ingredients
                .iter()
                .map(|nfi| nfi.range.clone())
                .collect(),
            available_ingredients,
        }))
    }

    fn overlap<T: PartialOrd + Copy>(
        r1: &RangeInclusive<T>,
        r2: &RangeInclusive<T>,
    ) -> Option<RangeInclusive<T>> {
        // r1 entirely contains r2
        // r1: *---*
        // r2:  *-*
        if r1.start() <= r2.start() && r1.end() >= r2.end() {
            return Some(r1.clone());
        }
        // r2 entirely contains r1
        // r1:  *-*
        // r2: *---*
        if r2.start() <= r1.start() && r2.end() >= r1.end() {
            return Some(r2.clone());
        }
        // r1 end contains r2 start
        // r1: *----*
        // r2:    *----*
        if r1.contains(r2.start()) && r2.end() > r1.end() {
            return Some(*r1.start()..=*r2.end());
        }
        // r1:    *----*
        // r2: *----*
        if r2.contains(r1.start()) && r1.end() > r2.end() {
            return Some(*r2.start()..=*r1.end());
        }
        None
    }
}

impl Solver for Day05 {
    fn part_one(&self) -> anyhow::Result<String> {
        Ok(self
            .available_ingredients
            .iter()
            .filter(|i| {
                self.normalised_fresh_ingredients
                    .iter()
                    .any(|r| r.contains(i))
            })
            .count()
            .to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        Ok(self
            .normalised_fresh_ingredients
            .iter()
            .map(|r| r.end() + 1 - r.start())
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
        let input = include_str!("../puzzles/day05/example.input")
            .lines()
            .map(String::from);

        let solver = Day05::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "3"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day05/example.input")
            .lines()
            .map(String::from);

        let solver = Day05::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "14"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day05/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day05::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "690"};
        assert_eq! {solver.part_two()?,  "344323629240733"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day05/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day05::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day05/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day05::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day05/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day05::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
