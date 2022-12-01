fn get_elf_calories(input: &str) -> Vec<u32> {
    let elves = input.split("\n\n");
    elves
        .map(|elf| {
            elf.split("\n").fold(0, |total, calories| {
                total + calories.parse::<u32>().unwrap()
            })
        })
        .collect()
}

pub fn part_one(input: &str) -> Option<u32> {
    get_elf_calories(input).into_iter().max()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut elf_calories = get_elf_calories(input);
    elf_calories.sort_by(|a, b| b.cmp(a));
    elf_calories
        .into_iter()
        .take(3)
        .fold(Some(0), |acc, item| Some(acc.unwrap_or(0) + item))
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
        assert_eq!(part_one(&input), Some(69281));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Some(201524));
    }
}
