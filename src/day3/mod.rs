use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn main() {
    let file_contents = fs::read_to_string("src/day3/input.txt")
    .expect("Error reading file");

    let priority_sum = part1(file_contents.clone());
    println!("Sum priority: {}", priority_sum);

    let priority_sum = part2(file_contents.clone());
    println!("Badge Sum priority: {}", priority_sum);
}

fn part1(file_contents: String) -> u32 {

    let mut misplaced_items = Vec::new();

    for rucksack_content in file_contents.lines() {
        let (compartment_1, compartment_2) = get_rucksack_compartments(rucksack_content.to_string());

        misplaced_items.push(
            get_priority(
                find_common_item(compartment_1, compartment_2)
            )
        );
    }

    return misplaced_items.iter().sum::<u32>();
}

fn part2(file_contents: String) -> u32 {

    let mut badge_priorities = Vec::new();

    let mut group_rucksacks = Vec::new();
    for (index, rucksack_content) in file_contents.lines().enumerate() {
        group_rucksacks.push(rucksack_content.to_string());

        if (index + 1) % 3 == 0 {
            badge_priorities.push(
                get_priority(
                    find_common_item_in_three_compartments(
                        group_rucksacks[0].clone(),
                        group_rucksacks[1].clone(),
                        group_rucksacks[2].clone()
                    )
                )
            );

            group_rucksacks.clear();
        }
    }

    return badge_priorities.iter().sum::<u32>();
}

fn get_rucksack_compartments(rucksack_content: String) -> (String, String) {
    let (compartment_1, compartment_2) = rucksack_content.split_at(rucksack_content.len() / 2);
    
    return (compartment_1.to_string(), compartment_2.to_string());
}

fn find_common_item(compartment_1: String, compartment_2: String) -> char {
    let compartment_1_chars: HashSet<char> = compartment_1.chars().collect();
    let compartment_2_chars: HashSet<char> = compartment_2.chars().collect();
    
    let common_chars: HashSet<char> = compartment_1_chars.intersection(&compartment_2_chars).cloned().collect();

    return *common_chars.iter().next().unwrap();
}

fn find_common_item_in_three_compartments(compartment_1: String, compartment_2: String, compartment_3: String) -> char {
    let compartment_1_chars: HashSet<char> = compartment_1.chars().collect();
    let compartment_2_chars: HashSet<char> = compartment_2.chars().collect();
    let compartment_3_chars: HashSet<char> = compartment_3.chars().collect();
    
    let common_chars: HashSet<char> = compartment_1_chars.intersection(&compartment_2_chars).cloned().collect();
    let common_chars: HashSet<char> = common_chars.intersection(&compartment_3_chars).cloned().collect();

    return *common_chars.iter().next().unwrap();
}

fn get_priority(item: char) -> u32 {
    let ascii_value = item as u32;

    if item.is_uppercase() {
        return ascii_value - 38;
    } else {
        return ascii_value - 96;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        fs::read_to_string("src/day3/input.txt")
        .expect("Error reading file")
    }

    #[test]
    fn test_part1() {
        let input = setup();

        assert_eq!(part1(input), 8243);
    }

    #[test]
    fn test_get_rucksack_compartments() {
        assert_eq!(
            get_rucksack_compartments("abcdef".to_string()),
            ("abc".to_string(), "def".to_string())
        );
    }

    #[test]
    fn test_get_priority() {
        assert_eq!(get_priority('a'), 1);
        assert_eq!(get_priority('p'), 16);
        assert_eq!(get_priority('P'), 42);
        assert_eq!(get_priority('L'), 38);
    }

    #[test]
    fn test_find_common_item() {
        assert_eq!(find_common_item("abc".to_string(), "ade".to_string()), 'a');
    }

    #[test]
    fn test_find_common_item_in_three_compartments() {
        assert_eq!(
            find_common_item_in_three_compartments("abc".to_string(), "ade".to_string(), "ade".to_string()), 'a');
    }
}
