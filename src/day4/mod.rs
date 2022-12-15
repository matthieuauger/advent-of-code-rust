use crate::_utils::read_input;

#[allow(dead_code)]
pub fn main() {
    let file_contents = read_input("src/day4/input.txt");

    let result = part1(&file_contents);
    println!("result: {}", result);

    let result = part2(&file_contents);
    println!("result: {}", result);
}

fn part1(file_contents: &String) -> u32 {

    let mut full_overlapped_assignations = Vec::new();
    for pair_assignations in file_contents.lines() {
        let (assignation_1, assignation_2) = get_assignations(pair_assignations.to_string());

        if are_assignations_overlapped(assignation_1.clone(), assignation_2.clone()) {
            full_overlapped_assignations.push(pair_assignations.to_string());
        }
    }

    return full_overlapped_assignations.iter().count() as u32;
}

fn part2(file_contents: &String) -> u32 {

    let mut overlapped_assignations_count = 0u32;
    for pair_assignations in file_contents.lines() {
        let (assignation_1, assignation_2) = get_assignations(pair_assignations.to_string());

        let count = get_assignations_overlapped_zones_count(
            assignation_1.clone(),
            assignation_2.clone()
        );

        if count > 0 {
            overlapped_assignations_count += 1;
        }
    }

    return overlapped_assignations_count;
}

fn get_assignations(assignation: String) -> (String, String) {
    let mut iter = assignation.split(|char| char == ',');
    
    return (iter.next().unwrap().to_string(), iter.next().unwrap().to_string());
}

fn are_assignations_overlapped(assignation_1: String, assignation_2: String) -> bool {
    let (assignation_1_start, assignation_1_end) = get_zones(assignation_1);
    let (assignation_2_start, assignation_2_end) = get_zones(assignation_2);

    if assignation_1_start <= assignation_2_start && assignation_1_end >= assignation_2_end {
        return true;
    }
    
    if assignation_2_start <= assignation_1_start && assignation_2_end >= assignation_1_end {
        return true;
    }
    
    return false;
}

fn get_assignations_overlapped_zones_count(assignation_1: String, assignation_2: String) -> u32 {
    let (assignation_1_start, assignation_1_end) = get_zones(assignation_1);
    let (assignation_2_start, assignation_2_end) = get_zones(assignation_2);

    if assignation_1_end < assignation_2_start {
        return 0;
    }

    if assignation_2_end < assignation_1_start {
        return 0;
    }

    if assignation_1_end == assignation_2_start {
        return 1;
    }

    if assignation_2_end == assignation_1_start {
        return 1;
    }

    if assignation_1_start <= assignation_2_start && assignation_1_end >= assignation_2_end {
        return assignation_2_end - assignation_2_start + 1;
    }
    
    if assignation_1_start <= assignation_2_start && assignation_1_end <= assignation_2_end {
        return assignation_1_end - assignation_2_start + 1;
    }
    
    if assignation_2_start <= assignation_1_start && assignation_2_end >= assignation_1_end {
        return assignation_1_end - assignation_1_start + 1;
    }

    if assignation_2_start <= assignation_1_start && assignation_2_end <= assignation_1_end {
        return assignation_2_end - assignation_1_start + 1;
    }

    return 0;
}

fn get_zones(assignation: String) -> (u32, u32) {
    let mut iter = assignation.split(|char| char == '-');
    
    return (iter.next().unwrap().parse::<u32>().unwrap(), iter.next().unwrap().parse::<u32>().unwrap());
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        read_input("src/day4/input.txt")
    }

    #[allow(dead_code)]
    fn test_part1() {
        let input = setup();

        assert_eq!(part1(&input), 8243);
    }

    #[test]
    fn test_get_assignations() {
        assert_eq!(
            get_assignations("2-4,6-8".to_string()),
            ("2-4".to_string(), "6-8".to_string())
        );
    }

    #[test]
    fn test_get_zones() {
        assert_eq!(get_zones("2-4".to_string()), (2, 4));
        assert_eq!(get_zones("12-14".to_string()), (12, 14));
    }
    
    #[test]
    fn test_are_assignations_overlapped() {
        assert_eq!(are_assignations_overlapped("2-4".to_string(), "3-5".to_string()), false);
        assert_eq!(are_assignations_overlapped("2-8".to_string(), "3-7".to_string()), true);
        assert_eq!(are_assignations_overlapped("6-6".to_string(), "4-6".to_string()), true);
    }    
    
    #[test]
    fn test_get_assignations_overlapped_zones_count() {
        assert_eq!(get_assignations_overlapped_zones_count("5-7".to_string(), "7-9".to_string()), 1);
        assert_eq!(get_assignations_overlapped_zones_count("2-8".to_string(), "3-7".to_string()), 5);
        assert_eq!(get_assignations_overlapped_zones_count("6-6".to_string(), "4-6".to_string()), 1);
        assert_eq!(get_assignations_overlapped_zones_count("2-6".to_string(), "4-8".to_string()), 3);
        assert_eq!(get_assignations_overlapped_zones_count("33-92".to_string(), "93-93".to_string()), 0);
    }
}
