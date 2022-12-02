use std::collections::HashMap;

pub fn part_one(input: &str) -> Option<i32> {
    input.lines().fold(Some(0), |acc, val| {
        let mut chars = val.chars();
        let sign: char = chars.next().unwrap();
        let magnitude: i32 = chars
            .fold(Some(0), |acc, digit| {
                Some(acc.unwrap() * 10 + digit.to_digit(10).unwrap() as i32)
            })
            .unwrap();
        match sign {
            '-' => Some(acc.unwrap() - magnitude),
            '+' => Some(acc.unwrap() + magnitude),
            _ => panic!(
                "Sign is {} and does not match the two options of + or - ",
                sign
            ),
        }
    })
}

pub fn part_two(input: &str) -> Option<i32> {
    let lines = input.lines();
    let mut acc: i32 = 0;
    let mut seen: HashMap<String, bool> = HashMap::new();
    for line in lines.cycle() {
        let mut chars = line.chars();
        let sign: char = chars.next().unwrap();
        let magnitude: i32 = chars
            .fold(Some(0), |acc, digit| {
                Some(acc.unwrap() * 10 + digit.to_digit(10).unwrap() as i32)
            })
            .unwrap();
        let next = match sign {
            '-' => acc - magnitude,
            '+' => acc + magnitude,
            _ => panic!(
                "Sign is {} and does not match the two options of + or - ",
                sign
            ),
        };
        let matches = match seen.contains_key(&next.to_string()) {
            true => Some(next),
            false => {
                seen.insert(next.to_string(), true);
                acc = next;
                None
            }
        };
        // println!("Value: {}, is seen? {:?}", next, matches);
        if matches.is_some() {
            return matches;
        }
    }
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Some(425));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(57538));
    }
}
