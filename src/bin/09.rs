use std::cmp::Ordering;

#[derive(Debug, Default, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn is_touching(&self, rhs: &Position) -> bool {
        self.x.abs_diff(rhs.x) <= 1 && self.y.abs_diff(rhs.y) <= 1
    }

    fn get_next_x(&self, rhs: &Position) -> isize {
        if self.x == rhs.x {
            0
        } else if self.x > rhs.x {
            -1
        } else {
            1
        }
    }

    fn get_next_y(&self, rhs: &Position) -> isize {
        if self.y == rhs.y {
            0
        } else if self.y > rhs.y {
            -1
        } else {
            1
        }
    }

    fn get_next_position(&self, rhs: &Position) -> Option<Position> {
        // If the head and tail are in the same row or column
        // the tail just has to move vertically or horizontally
        // otherwise, it moves diagonally

        if !self.is_touching(rhs) {
            let (next_x, next_y) = (self.get_next_x(rhs), self.get_next_y(rhs));
            Some(Position {
                x: self.x + next_x,
                y: self.y + next_y,
            })
        } else {
            None
        }
    }

    fn move_head(&self, dir: &str) -> Self {
        match dir {
            "U" => Position {
                x: self.x,
                y: self.y + 1,
            },
            "D" => Position {
                x: self.x,
                y: self.y - 1,
            },
            "L" => Position {
                x: self.x - 1,
                y: self.y,
            },
            "R" => Position {
                x: self.x + 1,
                y: self.y,
            },
            _ => panic!("Direction not one of the cardinal directions"),
        }
    }
}

impl PartialEq for Position {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl Eq for Position {
    fn assert_receiver_is_total_eq(&self) {}
}

impl PartialOrd for Position {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.x > other.x {
            Some(Ordering::Greater)
        } else if self.x == other.x {
            if self.y > other.y {
                Some(Ordering::Greater)
            } else if self.y == other.y {
                Some(Ordering::Equal)
            } else {
                Some(Ordering::Less)
            }
        } else {
            Some(Ordering::Less)
        }
    }
}
impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.x > other.x {
            Ordering::Greater
        } else if self.x == other.x {
            if self.y > other.y {
                Ordering::Greater
            } else if self.y == other.y {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Less
        }
    }
}

#[derive(Debug, Default)]
struct Knot {
    positions: Vec<Position>,
    current: Position,
}

impl Knot {
    fn move_head_next(&mut self, dir: &str) {
        let next = self.current.move_head(dir);
        self.positions.push(self.current);
        self.current = next;
    }

    fn move_tail_next(&mut self, rhs: &Position) {
        let next = self.current.get_next_position(rhs);
        match next {
            Some(n) => {
                self.positions.push(n);
                self.current = n;
            }
            None => (),
        }
    }

    fn get_num_unique_positions(&mut self) -> u32 {
        self.positions.sort();
        self.positions.dedup();
        self.positions.len() as u32
    }
}

#[derive(Debug, Default)]
struct Grid {
    head: Knot,
    tail: Knot,
}

#[derive(Debug, Default)]
struct MultiGrid {
    rope: [Knot; 10],
}

impl MultiGrid {
    fn move_head(&mut self, direction: &str, magnitude: usize) {
        for _ in 0..magnitude {
            let mut rope = self.rope.iter_mut();
            let mut prev = rope.next().unwrap();
            prev.move_head_next(direction);
            for next in rope {
                next.move_tail_next(&prev.current);
                prev = next;
            }
        }
    }

    fn get_num_unique_positions(&mut self) -> u32 {
        self.rope.last_mut().unwrap().get_num_unique_positions()
    }
}

// Refactor

struct LinkedList {
    segments: Vec<(i32, i32)>,
    visited: Vec<(i32, i32)>,
}

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(dir: &str) -> Self {
        match dir {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Failed to parse direction"),
        }
    }
    fn get_move(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl LinkedList {
    fn new(len: usize) -> Self {
        Self {
            segments: vec![(0, 0); len],
            visited: Vec::new(),
        }
    }

    fn make_move(&mut self, dir: &Direction) {
        let delta = dir.get_move();
        self.segments[0].0 += delta.0;
        self.segments[0].1 += delta.1;

        for i in 1..self.segments.len() {
            let prev = self.segments[i - 1];
            let current = self.segments[i];
            let row_diff = prev.0 - current.0;
            let col_diff = prev.1 - current.1;

            if row_diff.abs() > 1 || col_diff.abs() > 1 {
                if row_diff != 0 {
                    self.segments[i].0 += row_diff.signum();
                }
                if col_diff != 0 {
                    self.segments[i].1 += col_diff.signum();
                }
            } else {
                break;
            }
            if i == self.segments.len() - 1 {
                self.visited.push(self.segments[i]);
            }
        }
    }
    fn get_num_unique_positions(&mut self) -> u32 {
        self.visited.sort();
        self.visited.dedup();
        self.visited.len() as u32
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = Grid::default();
    let lines = input.lines();

    for line in lines {
        let (direction, magnitude) = line.split_once(" ").unwrap();

        for _ in 0..magnitude.parse::<usize>().unwrap() {
            grid.head.move_head_next(direction);
            grid.tail.move_tail_next(&grid.head.current);
        }
    }

    Some(grid.tail.get_num_unique_positions())
}

pub fn part_one_redux(input: &str) -> Option<u32> {
    let mut linked_list = LinkedList::new(2);

    for line in input.lines() {
        let (direction, magnitude) = line.split_once(" ").unwrap();
        let dir = Direction::new(direction);
        for _ in 0..magnitude.parse().unwrap() {
            linked_list.make_move(&dir);
        }
    }
    Some(linked_list.get_num_unique_positions())
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = MultiGrid::default();
    let lines = input.lines();

    for line in lines {
        let (direction, magnitude) = line.split_once(" ").unwrap();

        grid.move_head(direction, magnitude.parse::<usize>().unwrap());
    }

    Some(grid.get_num_unique_positions())
}
pub fn part_two_redux(input: &str) -> Option<u32> {
    let mut linked_list = LinkedList::new(10);

    for line in input.lines() {
        let (direction, magnitude) = line.split_once(" ").unwrap();
        let dir = Direction::new(direction);
        for _ in 0..magnitude.parse().unwrap() {
            linked_list.make_move(&dir);
        }
    }
    Some(linked_list.get_num_unique_positions())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(1, part_one_redux, input);
    advent_of_code::solve!(2, part_two, input);
    advent_of_code::solve!(2, part_two_redux, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
