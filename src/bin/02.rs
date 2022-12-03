enum Throw {
    Rock,
    Paper,
    Scissors,
}
enum Outcome {
    Lose = 0,
    Draw = 3,
    Win = 6,
}

impl Throw {
    fn opponent(t: char) -> Self {
        match t {
            'A' => Throw::Rock,
            'B' => Throw::Paper,
            'C' => Throw::Scissors,
            _ => panic!("Opponent throw did not fit the options."),
        }
    }
    fn own(t: char) -> Self {
        match t {
            'X' => Throw::Rock,
            'Y' => Throw::Paper,
            'Z' => Throw::Scissors,
            _ => panic!("Opponent throw did not fit the options."),
        }
    }

    fn find_opposite_throw(&self, outcome: Outcome) -> Self {
        match (self, outcome) {
            (Throw::Rock, Outcome::Lose) => Throw::Scissors,
            (Throw::Rock, Outcome::Draw) => Throw::Rock,
            (Throw::Rock, Outcome::Win) => Throw::Paper,
            (Throw::Paper, Outcome::Lose) => Throw::Rock,
            (Throw::Paper, Outcome::Draw) => Throw::Paper,
            (Throw::Paper, Outcome::Win) => Throw::Scissors,
            (Throw::Scissors, Outcome::Lose) => Throw::Paper,
            (Throw::Scissors, Outcome::Draw) => Throw::Scissors,
            (Throw::Scissors, Outcome::Win) => Throw::Rock,
        }
    }
}

impl Outcome {
    fn new(t: char) -> Self {
        match t {
            'X' => Outcome::Lose,
            'Y' => Outcome::Draw,
            'Z' => Outcome::Win,
            _ => panic!("Outcome not in the existing list"),
        }
    }
}

struct Round {
    opponent_throw: Throw,
    matching_throw: Throw,
}

impl Round {
    fn new(round_string: &str) -> Self {
        let mut split = round_string.chars();
        let opponent_throw = Throw::opponent(split.next().unwrap());
        let _ = split.next();
        let matching_throw = Throw::own(split.next().unwrap());
        Round {
            opponent_throw,
            matching_throw,
        }
    }

    fn new_part_two(round_string: &str) -> Self {
        let mut split = round_string.chars();
        let opponent_throw = Throw::opponent(split.next().unwrap());
        let _ = split.next();
        let outcome = Outcome::new(split.next().unwrap());
        let matching_throw = opponent_throw.find_opposite_throw(outcome);
        Round {
            opponent_throw,
            matching_throw,
        }
    }

    fn score_round(&self) -> u32 {
        let throw_score = match self.matching_throw {
            Throw::Rock => 1,
            Throw::Paper => 2,
            Throw::Scissors => 3,
        };
        let match_score = self.score_throw();
        throw_score + match_score
    }
    fn score_throw(&self) -> u32 {
        match (&self.matching_throw, &self.opponent_throw) {
            (Throw::Rock, Throw::Rock) => 3,
            (Throw::Rock, Throw::Paper) => 0,
            (Throw::Rock, Throw::Scissors) => 6,
            (Throw::Paper, Throw::Rock) => 6,
            (Throw::Paper, Throw::Paper) => 3,
            (Throw::Paper, Throw::Scissors) => 0,
            (Throw::Scissors, Throw::Rock) => 0,
            (Throw::Scissors, Throw::Paper) => 6,
            (Throw::Scissors, Throw::Scissors) => 3,
        }
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut score: u32 = 0;
    for line in lines {
        let round = Round::new(line);
        score += round.score_round();
    }
    Some(score)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut score: u32 = 0;
    for line in lines {
        let round = Round::new_part_two(line);
        score += round.score_round();
    }
    Some(score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), None);
    }
}
