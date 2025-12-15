extern crate test;

use z3::{Solver as Z3Solver, ast::Int, Optimize};

use solver::{Solver, SolverToAny};
use std::collections::{HashMap, HashSet, VecDeque};

pub struct Day10 {
    machines: Vec<Machine>,
}

#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltages: Vec<usize>,
}

impl Machine {
    fn initialise(&self) -> usize {
        let mut states = VecDeque::new();
        states.push_back((0, vec![false; self.lights.len()]));

        while let Some((n, current)) = states.pop_front() {
            if current == self.lights {
                return n;
            }
            for button in &self.buttons {
                let mut next = current.clone();
                for target in button {
                    next[*target] = !next[*target];
                }
                states.push_back((n + 1, next));
            }
        }

        panic! {"exhausted search space without finding solution"};
    }

    fn charge(&self) -> u64 {
        // represents the number of times that each button is pressed.
        let mut button_vars = Vec::new();
        let solver = Z3Solver::new();
        let opt = Optimize::new();

        let mut lights_to_buttons = HashMap::new();
        for i in 0..self.buttons.len() {
            button_vars.push(Int::fresh_const(&format! {"button_{i}"}));
            for light in &self.buttons[i] {
                lights_to_buttons
                    .entry(light)
                    .or_insert(HashSet::new())
                    .insert(i);
            }
            // Buttons can't be pressed a negative number of times.
            opt.assert(&button_vars[i].ge(0));
        }

        // Want to minimise the total number of button presses.
        opt.minimize(&button_vars.iter().sum::<Int>());

        for light in 0..self.joltages.len() {
            // Each light's joltage is the sum of the number of times that
            // relevant buttons were pressed.
            opt.assert(
                &lights_to_buttons[&light]
                    .iter()
                    .map(|i| &button_vars[*i])
                    .sum::<Int>()
                    .eq(self.joltages[light] as u64),
            );
        }

        if opt.check(&[]) == z3::SatResult::Sat {
            // if the model is satisfiable, then it will be defined.
            let model = opt.get_model().unwrap();

            return button_vars.iter().filter_map(|v| model.get_const_interp(v))
                .filter_map(|i| i.as_u64()).sum::<u64>();

        }
        panic!{"must be able to find a solution"};
    }

    fn charge_naive(&self) -> usize {
        let mut states = VecDeque::new();
        states.push_back((0, vec![0; self.lights.len()]));

        while let Some((n, current)) = states.pop_front() {
            if current == self.joltages {
                return n;
            }
            'button: for button in &self.buttons {
                let mut next = current.clone();
                for target in button {
                    next[*target] += 1;
                }
                for i in 0..self.lights.len() {
                    if next[i] > self.joltages[i] {
                        continue 'button;
                    }
                }
                states.push_back((n + 1, next));
            }
        }

        panic! {"exhausted search space without finding solution"};
    }
}

#[derive(PartialEq, Debug)]
enum ParseState {
    Start,
    Lights,
    LightsEnd,
    Buttons,
    Joltages,
    End,
}

impl From<String> for Machine {
    fn from(value: String) -> Self {
        let mut lights = Vec::new();
        let mut buttons = Vec::new();
        let mut joltages = Vec::new();
        let mut state = ParseState::Start;

        let mut acc = 0;
        let mut wires = Vec::new();
        for c in value.chars() {
            match state {
                ParseState::Start => match c {
                    '[' => {
                        state = ParseState::Lights;
                    }
                    _ => panic! {"unrecognised char {c} in {value}"},
                },
                ParseState::Lights => match c {
                    '.' => {
                        lights.push(false);
                    }
                    '#' => {
                        lights.push(true);
                    }
                    ']' => {
                        state = ParseState::LightsEnd;
                    }
                    _ => panic! {"unrecognised char {c} in {value}"},
                },
                ParseState::LightsEnd => match c {
                    ' ' => {}
                    '(' => {
                        state = ParseState::Buttons;
                    }
                    _ => panic! {"unrecognised char {c} in {value}"},
                },
                ParseState::Buttons => match c {
                    ' ' => {
                        assert_eq! {acc, 0};
                        assert! {wires.is_empty()}
                    }
                    '(' => {}
                    '0'..='9' => {
                        acc = (acc * 10) + (c as u8 - b'0') as usize;
                    }
                    ',' => {
                        assert! {acc < lights.len()};
                        wires.push(acc);
                        acc = 0;
                    }
                    ')' => {
                        assert! {acc < lights.len()};
                        wires.push(acc);
                        acc = 0;
                        buttons.push(wires);
                        wires = Vec::new();
                    }
                    '{' => {
                        state = ParseState::Joltages;
                    }
                    _ => panic! {"unrecognised char {c} in {value}"},
                },
                ParseState::Joltages => match c {
                    '0'..='9' => {
                        acc = (acc * 10) + (c as u8 - b'0') as usize;
                    }
                    ',' => {
                        joltages.push(acc);
                        acc = 0;
                    }
                    '}' => {
                        joltages.push(acc);
                        assert_eq! {lights.len(), joltages.len()};
                        state = ParseState::End;
                    }
                    _ => panic! {"unrecognised char {c} in {value}"},
                },
                ParseState::End => {
                    panic! {"should have no more chars to process"};
                }
            }
        }

        assert_eq! {state, ParseState::End};
        Machine {
            lights,
            buttons,
            joltages,
        }
    }
}

impl SolverToAny for Day10 {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Day10 {
    pub fn try_create(input: Box<dyn Iterator<Item = String>>) -> anyhow::Result<Box<dyn Solver>> {
        Ok(Box::new(Day10 {
            machines: input.filter(|l| !l.is_empty()).map(Machine::from).collect(),
        }))
    }
}

impl Solver for Day10 {
    fn part_one(&self) -> anyhow::Result<String> {
        Ok(self
            .machines
            .iter()
            .map(Machine::initialise)
            .sum::<usize>()
            .to_string())
    }

    fn part_two(&self) -> anyhow::Result<String> {
        Ok(self
            .machines
            .iter()
            .map(Machine::charge)
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
        let input = include_str!("../puzzles/day10/example.input")
            .lines()
            .map(String::from);

        let solver = Day10::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "7"};
        Ok(())
    }

    #[test]
    fn it_works_on_the_other_example() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day10/example.input")
            .lines()
            .map(String::from);

        let solver = Day10::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_two()?, "33"};
        Ok(())
    }

    #[test]
    fn it_works_on_my_input() -> Result<(), Box<dyn Error>> {
        let input = include_str!("../puzzles/day10/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day10::try_create(Box::new(input)).unwrap();
        assert_eq! {solver.part_one()?, "457"};
        assert_eq! {solver.part_two()?, "17576"};
        Ok(())
    }

    #[bench]
    fn bench_parse(b: &mut Bencher) {
        b.iter(|| {
            let input = include_str!("../puzzles/day10/joshua.input")
                .lines()
                .map(String::from);

            let _solver = Day10::try_create(Box::new(input)).unwrap();
        });
    }

    #[bench]
    fn bench_one(b: &mut Bencher) {
        let input = include_str!("../puzzles/day10/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day10::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_one());
    }

    #[ignore]
    #[bench]
    fn bench_two(b: &mut Bencher) {
        let input = include_str!("../puzzles/day10/joshua.input")
            .lines()
            .map(String::from);

        let solver = Day10::try_create(Box::new(input)).unwrap();

        b.iter(|| solver.part_two());
    }
}
