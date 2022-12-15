use crate::_utils::read_input;

#[allow(dead_code)]
pub fn main() {
    let file_contents = read_input("src/day6/input.txt");

    part1(&file_contents);
    part2(&file_contents);
}

fn part1(file_contents: &String) -> u32 {

    let file_char = file_contents.chars().collect::<Vec<char>>();
    for (index, _char) in file_char.iter().enumerate() {
        
        if index >= 3 {
            let mut buffer = Vec::new();

            for i in 0..4 {
                buffer.push(file_char[index - i]);
            }

            if !has_duplicates(buffer) {
                println!("Index is {}", index + 1);
                break;
            }
        }
    }

    return 0;
}

fn part2(file_contents: &String) -> u32 {

    let file_char = file_contents.chars().collect::<Vec<char>>();
    for (index, _char) in file_char.iter().enumerate() {
        
        if index >= 13 {
            let mut buffer = Vec::new();

            for i in 0..14 {
                buffer.push(file_char[index - i]);
            }

            if !has_duplicates(buffer) {
                println!("Index is {}", index + 1);
                break;
            }
        }
    }

    return 0;
}

fn has_duplicates(v: Vec<char>) -> bool {
    for i in 0..v.len() {
        for j in 0..v.len() {
            if i != j && v[i] == v[j] {
                return true;
            }
        }
    }
    return false;
}


#[cfg(test)]
mod tests {
    use crate::_test_utils::setup;
    use super::*;

    fn _setup() -> String {
        read_input("src/day6/input.txt")
    }

    #[test]
    fn test_true() {
        setup();
        assert_eq!(1, 1);
    }
}
