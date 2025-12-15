extern crate test;

use std::{
    collections::{BTreeSet, HashMap, HashSet},
    ops::RangeInclusive,
};

use solver::{Solver, SolverToAny};

pub struct Day09 {
    points: Vec<Point>,
    min_x: u64,
    min_y: u64,
    max_x: u64,
    max_y: u64,
}

#[derive(Eq, PartialEq, Hash, Debug, Clone, Copy)]
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

    fn between(&self, other: &Self) -> Vec<Point> {
        if self.horizontal(other) {
            let min = self.x.min(other.x);
            let max = self.x.max(other.x);
            (min..=max).map(|x| Point { x, y: self.y }).collect()
        } else {
            let min = self.y.min(other.y);
            let max = self.y.max(other.y);
            (min..=max).map(|x| Point { x, y: self.y }).collect()
        }
    }
}

impl SolverToAny for Day09 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day09 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        let points = input
            .filter(|s| !s.is_empty())
            .map(Point::from)
            .collect::<Vec<_>>();
        let mut min_x = u64::MAX;
        let mut min_y = u64::MAX;
        let mut max_x = 0;
        let mut max_y = 0;
        for point in &points {
            min_x = point.x.min(min_x);
            min_y = point.y.min(min_y);
            max_x = point.x.max(max_x);
            max_y = point.y.max(max_y);
        }
        println! {
        "puzzle size: x: {}-{}, y: {}-{}, area: {}",
        min_x, max_x, min_y, max_y,
        Point{x: min_x, y: min_y}.rect(&Point{x: max_x, y: max_y})};

        Ok(Box::new(Day09 {
            points,
            min_x,
            min_y,
            max_x,
            max_y,
        }))
    }

    fn is_line_in_shape(
        &self,
        start: &Point,
        end: &Point,
        vertical: &HashMap<&u64, Vec<RangeInclusive<u64> > >, // Vertical lines
        horizontal: &HashMap<&u64, Vec<RangeInclusive<u64> > >, // Horizontal lines
    ) -> bool {
        // A line is within the shape if it is a series of points within a
        // line, optionally followed by a series of points outside the line and
        // a series of points within another line.

        if start.horizontal(end) {
            let line_segments = &horizontal[&start.y];
            let range = range(start.x, end.x);
            let i = range.start();
            let mut j = 0;
            while i < range.end() {
                if line_segments[j].start() < i {
                    j += 1;
                } else if line_segments[j].start() == i {

                } else if line_segments[j].end() == i {

                }
            }
            return i == range.end();
        } else {
            let line_segments = &vertical[&start.x];
            let range = range(start.y, end.y);
        }
        false
    }
}

fn range<T: Ord + Copy>(l: T, r: T) -> RangeInclusive<T> {
    let min = l.min(r);
    let max = l.max(r);
    min..=max
}

impl Solver for Day09 {
    fn part_one(&self) -> anyhow::Result<String> {
        let mut largest_area = 0;
        for i in 0..self.points.len() {
            // Skip the next point - the area will just be a straight line.
            for j in i + 2..self.points.len() {
                largest_area = largest_area.max(self.points[i].rect(&self.points[j]));
            }
        }
        Ok(largest_area.to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        let mut vertical = HashMap::new();
        let mut horizontal = HashMap::new();
        for i in 0..self.points.len() {
            let j = (i + 1) % self.points.len();
            if self.points[i].horizontal(&self.points[j]) {
                horizontal
                    .entry(&self.points[i].y)
                    .or_insert(Vec::new())
                    .push(range(self.points[i].x, self.points[j].x));
            } else {
                vertical
                    .entry(&self.points[i].x)
                    .or_insert(Vec::new())
                    .push(range(self.points[i].y, self.points[j].y));
            }
        }
        let points_hash = &self.points.iter().collect::<HashSet<_>>();
        let mut largest_area = 0;
        for i in 0..self.points.len() {
            // Skip the next point - the area is just a straight line
            for j in i + 2..self.points.len() {
                let opposite_1 = Point {
                    x: self.points[i].x,
                    y: self.points[j].y,
                };
                let opposite_2 = Point {
                    x: self.points[j].x,
                    y: self.points[i].y,
                };

                // if [
                //     &self.points[i],
                //     &opposite_1,
                //     &self.points[j],
                //     &opposite_2
                // ].iter().all(|p| self.is_point_in_shape(p, &vertical, &horizontal, &mut seen)) {
                //     largest_area = largest_area.max(self.points[i].rect(&self.points[j]));
                // }

                if [
                    (self.points[i],&opposite_1),
                    (opposite_1, &self.points[j]),
                    (self.points[j], &opposite_2),
                    (opposite_2, &self.points[i]),
                ]
                .iter()
                .all(|(l,r)| self.is_line_in_shape(l, r, &vertical, &horizontal))
                {
                    largest_area = largest_area.max(self.points[i].rect(&self.points[j]));
                }
            }
        }
        Ok(largest_area.to_string())
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
    #[ignore]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day09/example.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "24"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day09/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "4755429952"};
        // 452333056 too low
        // assert_eq! {solver.part_two()?, ""};
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
    #[ignore]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day09/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day09::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
