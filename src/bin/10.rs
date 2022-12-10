use itertools::Itertools;

struct Machine {
    cycles: Vec<isize>,
    x: isize,
}

impl Machine {
    fn new() -> Self {
        let mut cycles = Vec::new();
        cycles.push(1);
        Machine { cycles, x: 1 }
    }

    fn process_instruction(&mut self, instr: &str) {
        let mut split = instr.split_whitespace();
        match split.next().unwrap() {
            "noop" => self.cycles.push(self.x),
            "addx" => {
                self.cycles.push(self.x);
                self.cycles.push(self.x);
                self.x += split.next().unwrap().parse::<isize>().unwrap();
            }
            _ => panic!("Unknown instruction!"),
        }
    }

    fn get_strength(&self, index: usize) -> isize {
        self.cycles.get(index).unwrap() * index as isize
    }

    fn get_strengths(&self) -> isize {
        self.get_strength(20)
            + self.get_strength(60)
            + self.get_strength(100)
            + self.get_strength(140)
            + self.get_strength(180)
            + self.get_strength(220)
    }

    fn render(&self) -> String {
        // Loop through the set of cycles.  If the register is within one of the pixel being drawn,
        // the pixel is considered 'on' and rendered with a '#', otherwise '.'
        let mut cycles = self.cycles.iter();
        let _discard_first = cycles.next();
        cycles
            .enumerate()
            .map(|(index, x_val)| {
                let currently_rendering = index % 40;
                if (currently_rendering as isize).abs_diff(*x_val) <= 1 {
                    '#'
                } else {
                    '.'
                }
            })
            .chunks(40)
            .into_iter()
            .map(|line| line.collect::<String>())
            .fold(String::new(), |acc, val| acc + &val[..] + "\n")
    }
}

pub fn part_one(input: &str) -> Option<isize> {
    let mut machine = Machine::new();
    for line in input.lines() {
        machine.process_instruction(line);
    }
    Some(machine.get_strengths())
}

pub fn part_two(input: &str) -> Option<String> {
    let mut machine = Machine::new();
    for line in input.lines() {
        machine.process_instruction(line);
    }
    Some(machine.render())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);
        assert_eq!(part_two(&input), None);
    }
}
