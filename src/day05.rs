extern crate test;

use anyhow::Context;
use solver::{Solver, SolverToAny};
use std::collections::VecDeque;
use std::ops::RangeInclusive;

pub struct Day05 {
    fresh_ingredients: VecDeque<RangeInclusive<u64>>,
    available_ingredients: Vec<u64>,
}

impl SolverToAny for Day05 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day05 {
    pub fn try_create(
        mut input: Box<dyn Iterator<Item = String>>,
    ) -> anyhow::Result<Box<dyn Solver>> {
        let mut fresh_ingredients = VecDeque::new();
        for line in input.by_ref() {
            if line.is_empty() {
                break;
            }
            if let Some((start, end)) = line.split_once('-')
                && let Ok(parsed_start) = start
                    .parse::<u64>()
                    .with_context(|| format! {"could not parse '{start}'"})
                && let Ok(parsed_end) = end
                    .parse::<u64>()
                    .with_context(|| format! {"could not parse '{end}'"})
            {
                let mut range = parsed_start..=parsed_end;

                let n = fresh_ingredients.len();
                let lower =
                    Day05::lin_search(&fresh_ingredients, *range.start(), |r| r.end() + 1, 0);
                let upper =
                    Day05::lin_search(&fresh_ingredients, range.end() + 1, |r| *r.start(), lower);

                match (lower, upper) {
                    (l, r) if l == n && r == n => {
                        fresh_ingredients.push_back(range);
                        continue;
                    }
                    (l, r) if r == n => {
                        range = *fresh_ingredients[l].start().min(&parsed_start)
                            ..=*fresh_ingredients[n - 1].end().max(&parsed_end);
                        for _ in l..r {
                            fresh_ingredients.pop_back();
                        }
                        fresh_ingredients.push_back(range);
                    }
                    (l, r) if l == r => {
                        fresh_ingredients.insert(l, range);
                    }
                    (l, r) => {
                        let new_min = *fresh_ingredients[l].start().min(&parsed_start);
                        let idx = if r == n { r - 1 } else { r };
                        let new_max = *fresh_ingredients[idx - 1].end().max(&parsed_end);
                        assert! {new_min < new_max, "{} < {}", new_min, new_max};
                        range = new_min..=new_max;
                        for _ in l..r {
                            fresh_ingredients.remove(l);
                        }
                        fresh_ingredients.insert(l, range);
                        continue;
                    }
                }
            } else {
                anyhow::bail! {"could not parse '{line}'"}
            }
        }

        assert! {fresh_ingredients.iter().is_sorted_by_key(|r| *r.start())};
        assert! {fresh_ingredients.iter().is_sorted_by_key(|r| *r.end())};

        let mut available_ingredients = Vec::new();
        for line in input {
            if line.is_empty() {
                break;
            }
            let ingredient = line
                .parse::<u64>()
                .with_context(|| format! {"could not parse '{line}'"})?;
            available_ingredients.push(ingredient);
        }
        available_ingredients.sort();

        Ok(Box::new(Day05 {
            fresh_ingredients,
            available_ingredients,
        }))
    }

    fn lin_search(
        vals: &VecDeque<RangeInclusive<u64>>,
        target: u64,
        key: fn(&RangeInclusive<u64>) -> u64,
        skip: usize,
    ) -> usize {
        for (idx, val) in vals.iter().enumerate().skip(skip) {
            if key(val) >= target {
                return idx;
            }
        }
        vals.len()
    }

    fn in_range(&self, ingredient: &u64, starting_index: &usize) -> (bool, usize) {
        let n = self.fresh_ingredients.len();

        for i in *starting_index..n {
            let range = &self.fresh_ingredients[i];
            if range.contains(ingredient) {
                return (true, i);
            }
            if range.start() > ingredient {
                return (false, i.checked_sub(1).unwrap_or(0));
            }
        }
        return (false, n);
    }
}

impl Solver for Day05 {
    fn part_one(&self) -> anyhow::Result<String> {
        let mut count = 0;
        let mut idx: usize = 0;

        // available_ingredients already sorted
        for ingredient in &self.available_ingredients {
            let (found, i) = self.in_range(ingredient, &idx);
            if found {
                count += 1;
            }
            idx = i;
        }
        Ok(count.to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        Ok(self
            .fresh_ingredients
            .iter()
            .map(|r| 1 + r.end() - r.start())
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
        assert_eq! {solver.part_two()?, "344323629240733"};
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
