extern crate test;

use solver::{Solver, SolverToAny};

pub struct Day09 {
    points: Vec<Point>,
}

struct Point {
    x: u64,
    y: u64,
}

impl From<String> for Point {
    fn from(value: String) -> Self {
        let Some((l, r)) = value.split_once(',') else {
            panic! {"could not split {value:?}"};
        };
        let x = l.parse::<u64>().unwrap();
        let y = r.parse::<u64>().unwrap();
        Point { x, y }
    }
}

impl Point {
    fn rect(&self, other: &Self) -> u64 {
        (self.x.abs_diff(other.x) + 1) * (self.y.abs_diff(other.y) + 1)
    }

    fn horizontal(&self, other: &Self) -> bool {
        self.y == other.y
    }
}

impl SolverToAny for Day09 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day09 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        Ok(Box::new(Day09 {
            points: input
                .filter(|s| !s.is_empty())
                .map(Point::from)
                .collect::<Vec<_>>(),
        }))
    }
}

impl Solver for Day09 {
    fn part_one(&self) -> anyhow::Result<String> {
        let mut largest_area = 0;
        for i in 0..self.points.len() {
            for j in i + 1..self.points.len() {
                largest_area = largest_area.max(self.points[i].rect(&self.points[j]));
            }
        }
        Ok(largest_area.to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        let mut min_x = u64::MAX;
        let mut min_y = u64::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        for point in &self.points {
            min_x = point.x.min(min_x);
            min_y = point.y.min(min_y);
            max_x = point.x.max(max_x);
            max_y = point.y.max(max_y);
        }
        let mut vertical = Vec::new();
        let mut horizontal = Vec::new();
        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();
            if self.points[i].horizontal(&self.points[j]) {
                horizontal.push((&self.points[i], &self.points[j]));
            } else {
                vertical.push((&self.points[i], &self.points[j]));
            }
        }
        let mut largest_area = 0;
        for point in &self.points {

        }
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
        let input = include_str!("../puzzles/day09/example.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "50"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day09/example.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();
        assert! {solver.part_two().is_err()};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day09/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "4755429952"};
        assert! {solver.part_two().is_err()};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day09/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day09::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day09/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day09/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
