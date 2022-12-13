use std::fs;
use std::collections::HashMap;

#[allow(dead_code)]
pub fn main() {
    let file_contents = fs::read_to_string("src/day2/input.txt")
    .expect("Error reading file");

    let my_score = part1(file_contents.clone());
    println!("Total score: {}", my_score);

    let my_score = part2(file_contents.clone());
    println!("Total score: {}", my_score);
}

#[derive(Eq, PartialEq, Hash)]
enum RockPaperScissors {
    Rock,
    Paper,
    Scissors,
}

#[derive(Eq, PartialEq, Hash)]
enum Result {
    Win,
    Lose,
    Draw,
}

fn part1(file_contents: String) -> i32 {
    use crate::day2::RockPaperScissors::{Paper, Rock, Scissors};
    use crate::day2::Result::{Draw, Lose, Win};

    let mut adversary_guideletter_mapping = HashMap::new();
    adversary_guideletter_mapping.insert('A', Rock);
    adversary_guideletter_mapping.insert('B', Paper);
    adversary_guideletter_mapping.insert('C', Scissors);

    let mut me_guideletter_mapping = HashMap::new();
    me_guideletter_mapping.insert('X', Rock);
    me_guideletter_mapping.insert('Y', Paper);
    me_guideletter_mapping.insert('Z', Scissors);

    let mut hand_score_mapping = HashMap::new();
    hand_score_mapping.insert(Rock, 1);
    hand_score_mapping.insert(Paper, 2);
    hand_score_mapping.insert(Scissors, 3);

    let mut party_result_mapping = HashMap::new();
    party_result_mapping.insert(Win, 6);
    party_result_mapping.insert(Draw, 3);
    party_result_mapping.insert(Lose, 0);

    let mut my_score = 0;
    for line in file_contents.lines() {
        let characters: Vec<char> = line.chars().collect();

        let hands = (
            adversary_guideletter_mapping.get(&characters[0]).unwrap(),
            me_guideletter_mapping.get(&characters[2]).unwrap()
        );

        match hands {
            (Rock, Rock) => my_score += party_result_mapping.get(&Draw).unwrap() + hand_score_mapping.get(&Rock).unwrap(),
            (Rock, Paper) => my_score += party_result_mapping.get(&Win).unwrap() + hand_score_mapping.get(&Paper).unwrap(),
            (Rock, Scissors) => my_score += party_result_mapping.get(&Lose).unwrap() + hand_score_mapping.get(&Scissors).unwrap(),
            (Paper, Rock) => my_score += party_result_mapping.get(&Lose).unwrap() + hand_score_mapping.get(&Rock).unwrap(),
            (Paper, Paper) => my_score += party_result_mapping.get(&Draw).unwrap() + hand_score_mapping.get(&Paper).unwrap(),
            (Paper, Scissors) => my_score += party_result_mapping.get(&Win).unwrap() + hand_score_mapping.get(&Scissors).unwrap(),
            (Scissors, Rock) => my_score += party_result_mapping.get(&Win).unwrap() + hand_score_mapping.get(&Rock).unwrap(),
            (Scissors, Paper) => my_score += party_result_mapping.get(&Lose).unwrap() + hand_score_mapping.get(&Paper).unwrap(),
            (Scissors, Scissors) => my_score += party_result_mapping.get(&Draw).unwrap() + hand_score_mapping.get(&Scissors).unwrap(),
        }

    }

    return my_score;
}

fn part2(file_contents: String) -> i32 {
    use crate::day2::RockPaperScissors::{Paper, Rock, Scissors};
    use crate::day2::Result::{Draw, Lose, Win};

    let mut adversary_guideletter_mapping = HashMap::new();
    adversary_guideletter_mapping.insert('A', Rock);
    adversary_guideletter_mapping.insert('B', Paper);
    adversary_guideletter_mapping.insert('C', Scissors);

    let mut me_guideletter_mapping = HashMap::new();
    me_guideletter_mapping.insert('X', Lose);
    me_guideletter_mapping.insert('Y', Draw);
    me_guideletter_mapping.insert('Z', Win);

    let mut hand_score_mapping = HashMap::new();
    hand_score_mapping.insert(Rock, 1);
    hand_score_mapping.insert(Paper, 2);
    hand_score_mapping.insert(Scissors, 3);

    let mut party_result_mapping = HashMap::new();
    party_result_mapping.insert(Win, 6);
    party_result_mapping.insert(Draw, 3);
    party_result_mapping.insert(Lose, 0);

    let mut my_score = 0;
    for line in file_contents.lines() {
        let characters: Vec<char> = line.chars().collect();
        let my_hand: RockPaperScissors;

        match (characters[0], characters[2]) {
            ('A', 'X') => my_hand = Scissors,
            ('A', 'Y') => my_hand = Rock,
            ('A', 'Z') => my_hand = Paper,
            ('B', 'X') => my_hand = Rock,
            ('B', 'Y') => my_hand = Paper,
            ('B', 'Z') => my_hand = Scissors,
            ('C', 'X') => my_hand = Paper,
            ('C', 'Y') => my_hand = Scissors,
            ('C', 'Z') => my_hand = Rock,
            _ => panic!("Invalid input")
        }

        let hands = (
            adversary_guideletter_mapping.get(&characters[0]).unwrap(),
            my_hand
        );

        match hands {
            (Rock, Rock) => my_score += party_result_mapping.get(&Draw).unwrap() + hand_score_mapping.get(&Rock).unwrap(),
            (Rock, Paper) => my_score += party_result_mapping.get(&Win).unwrap() + hand_score_mapping.get(&Paper).unwrap(),
            (Rock, Scissors) => my_score += party_result_mapping.get(&Lose).unwrap() + hand_score_mapping.get(&Scissors).unwrap(),
            (Paper, Rock) => my_score += party_result_mapping.get(&Lose).unwrap() + hand_score_mapping.get(&Rock).unwrap(),
            (Paper, Paper) => my_score += party_result_mapping.get(&Draw).unwrap() + hand_score_mapping.get(&Paper).unwrap(),
            (Paper, Scissors) => my_score += party_result_mapping.get(&Win).unwrap() + hand_score_mapping.get(&Scissors).unwrap(),
            (Scissors, Rock) => my_score += party_result_mapping.get(&Win).unwrap() + hand_score_mapping.get(&Rock).unwrap(),
            (Scissors, Paper) => my_score += party_result_mapping.get(&Lose).unwrap() + hand_score_mapping.get(&Paper).unwrap(),
            (Scissors, Scissors) => my_score += party_result_mapping.get(&Draw).unwrap() + hand_score_mapping.get(&Scissors).unwrap(),
        }

    }

    return my_score;
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        fs::read_to_string("src/day2/input.txt")
        .expect("Error reading file")
    }

    #[test]
    fn test_part1() {
        let input = setup();

        assert_eq!(part1(input), 15691);
    }

    #[test]
    fn test_part2() {
        let input = setup();

        assert_eq!(part2(input), 12989);
    }
}
