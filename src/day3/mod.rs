use std::{fs, collections::HashSet};

#[allow(dead_code)]
pub fn main() {
    let file_contents = fs::read_to_string("src/day3/input.txt")
    .expect("Error reading file");

    let priority_sum = part1(file_contents.clone());
    println!("Sum priority: {}", priority_sum);
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

    fn _setup() -> String {
        fs::read_to_string("src/day3/input.txt")
        .expect("Error reading file")
    }

    fn _test_part1() {
        let input = _setup();

        assert_eq!(part1(input), 0);
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
}
