extern crate test;

use std::{
    collections::VecDeque,
};

use solver::{Solver, SolverToAny};

pub struct Day09 {
    points: Vec<Point>,
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
        Ok(Box::new(Day09 {
            points,
        }))
    }

    fn clockwise_area(&self) -> i64 {
        let mut area = 0_i64;
        for i in 1..self.points.len() {
            let a = &self.points[i - 1];
            let b = &self.points[i];
            if a.x != b.x {
                area -= (a.y as i64) * (b.x as i64 - a.x as i64);
            }
        }
        area
    }
}

static GRID_LIMIT: usize = 100000;
static CONDENSED_GRID_LIMIT: usize = 1000;

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
        let mut xs = vec![0; GRID_LIMIT];
        let mut ys = vec![0; GRID_LIMIT];
        let mut grid = vec![vec!['.'; CONDENSED_GRID_LIMIT]; CONDENSED_GRID_LIMIT];

        let n = self.points.len();
        for i in 0..n {
            let p = &self.points[i];
            xs[p.x as usize] = 1;
            ys[p.y as usize] = 1;
        }

        let mut size_x = 0;
        let mut size_y = 0;

        for i in 0..GRID_LIMIT {
            if xs[i] == 1 {
                xs[i] = size_x + 1;
                size_x += 2;
            } else {
                xs[i] = size_x;
            }
            if ys[i] == 1 {
                ys[i] = size_y + 1;
                size_y += 2;
            } else {
                ys[i] = size_y;
            }
        }
        size_x += 1;
        size_y += 1;
        assert! {size_x <= CONDENSED_GRID_LIMIT};
        assert! {size_y <= CONDENSED_GRID_LIMIT};

        let is_clockwise = self.clockwise_area() > 0;

        for i in 0..n {
            let a = &self.points[i.checked_sub(1).unwrap_or(n - 1)];
            let b = &self.points[i];
            if a.x == b.x {
                let x = xs[a.x as usize];
                let ay = ys[a.y as usize];
                let by = ys[b.y as usize];
                let step: isize = if ay <= by { 1 } else { -1 };
                let end = by.strict_add_signed(step);
                let inside_offset: isize = if is_clockwise { -step } else { step };
                let mut y = ay;
                let xoff = x.strict_add_signed(inside_offset);
                while y != end {
                    grid[y][x] = '#';
                    if grid[y][xoff] == '.' {
                        grid[y][xoff] = '!';
                    }
                    y = y.strict_add_signed(step);
                }
            } else {
                let y = ys[a.y as usize];
                let ax = xs[a.x as usize];
                let bx = xs[b.x as usize];
                let step: isize = if ax <= bx { 1 } else { -1 };
                let end = bx.strict_add_signed(step);
                let inside_offset: isize = if is_clockwise { step } else { -step };
                let mut x = ax;
                while x != end {
                    grid[y][x] = '#';
                    let yoff = y.strict_add_signed(inside_offset);
                    if grid[yoff][x] == '.' {
                        grid[yoff][x] = '!';
                    }
                    x = x.strict_add_signed(step);
                }
            }
        }

        let mut queue = VecDeque::new();
        for y in 0..size_y {
            for x in 0..size_x {
                if grid[y][x] == '!' {
                    grid[y][x] = '#';
                    queue.push_back((x, y));
                }
            }
        }

        while let Some((x, y)) = queue.pop_front() {
            let neighbours = vec![
                (Some(x), y.checked_sub(1)),
                (
                    Some(x),
                    y.checked_add(1).filter(|y| *y < CONDENSED_GRID_LIMIT),
                ),
                (x.checked_sub(1), Some(y)),
                (
                    x.checked_add(1).filter(|x| *x < CONDENSED_GRID_LIMIT),
                    Some(y),
                ),
            ];

            for p in neighbours {
                match p {
                    (Some(nx), Some(ny)) => {
                        if grid[ny][nx] == '.' {
                            grid[ny][nx] = '#';
                            queue.push_back((nx, ny));
                        }
                    }
                    _ => {}
                }
            }
        }

        let mut largest_area = 0;
        for i in 0..n {
            let a = &self.points[i];
            let ax = xs[a.x as usize];
            let ay = ys[a.y as usize];
            'inner: for j in (i + 1)..n {
                let b = &self.points[j];
                let bx = xs[b.x as usize];
                let by = ys[b.y as usize];
                let min_x = if ax < bx { ax } else { bx };
                let max_x = if ax < bx { bx } else { ax };
                let min_y = if ay < by { ay } else { by };
                let max_y = if ay < by { by } else { ay };

                for y in min_y..=max_y {
                    if grid[y][min_x] != '#' || grid[y][max_x] != '#' {
                        continue 'inner;
                    }
                }
                for x in min_x..=max_x {
                    if grid[min_y][x] != '#' || grid[max_y][x] != '#' {
                        continue 'inner;
                    }
                }

                largest_area = largest_area.max(a.rect(b));
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
        assert_eq! {solver.part_two()?, "1429596008"};
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
