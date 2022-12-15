use crate::_utils::read_input;
use log::debug;
use std::str;

#[allow(dead_code)]
pub fn main() {
    let file_contents = read_input("src/day5/input.txt");

    part1(&file_contents);
}

#[derive(Debug)]
struct Instructions {
    crate_count: u16,
    from: u16,
    to: u16,
}

fn part1(file_contents: &String) -> u32 {
    let mut crate_file_contents = String::new();

    let mut crate_stacks: Vec<Vec<char>> = Vec::new();
    let mut instruction_reached = false;
    for line in file_contents.lines() {
        if line.is_empty() {
            instruction_reached = true;
            crate_stacks = fill_crate_stacks(&crate_file_contents);

            continue;
        }

        // first part of file
        if !instruction_reached {
            crate_file_contents.push_str(line);
            crate_file_contents.push('\n');
        } else {
            let instructions = read_instructions(line);
            crate_stacks = move_stack_9001(&crate_stacks, instructions);
        }
    }

    crate_stacks.iter().for_each(|stack| {
        print!("{:?}", stack.last().unwrap());
    });

    return 0;
}

fn fill_crate_stacks(file_contents: &String) -> Vec<Vec<char>> {
    let mut crate_stacks: Vec<Vec<char>> = Vec::new();

    for (level_index, stack_level) in file_contents.lines().rev().enumerate() {
        debug!("Level {}: {}", level_index, stack_level);
        
        // line with the number of crates
        if level_index == 0 {
            continue;
        }

        let stack_content = stack_level
            .as_bytes()
            .chunks(4)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap()
        ;

        for (crate_index, box_crate) in stack_content.iter().enumerate() {

            // first level of crates
            if level_index == 1 {
                crate_stacks.push(Vec::new());
            }

            match box_crate.chars().nth(1) {
                Some(crate_content) => {
                    if !crate_content.to_string().trim().is_empty() {
                        crate_stacks[crate_index].push(crate_content);
                    }
                },
                _ => continue,
            }
        }
    }

    return crate_stacks;
}

fn read_instructions(line: &str) -> Instructions {
    let mut instructions = Instructions {
        crate_count: 0,
        from: 0,
        to: 0,
    };

    for (index, instruction) in line.split_whitespace().enumerate() {
        match index {
            1 => instructions.crate_count = instruction.parse::<u16>().unwrap(),
            3 => instructions.from = instruction.parse::<u16>().unwrap(),
            5 => instructions.to = instruction.parse::<u16>().unwrap(),
            _ => continue,
        }
    }

    return instructions;
}

#[allow(dead_code)]
fn move_stack(crate_stacks: &Vec<Vec<char>>, instructions: Instructions) -> Vec<Vec<char>> {
    
    let mut updated_crate_stacks = crate_stacks.clone();

    for _n in 0..instructions.crate_count {
        let from_crate_stack = updated_crate_stacks.get_mut(usize::from(instructions.from - 1)).unwrap();
        let from_crate_stack_last_element = from_crate_stack.pop().unwrap();

        let to_crate_stack = updated_crate_stacks.get_mut(usize::from(instructions.to - 1)).unwrap();
        to_crate_stack.push(from_crate_stack_last_element);
    }
    
    return updated_crate_stacks;
}

#[allow(dead_code)]
fn move_stack_9001(crate_stacks: &Vec<Vec<char>>, instructions: Instructions) -> Vec<Vec<char>> {
    
    let mut updated_crate_stacks = crate_stacks.clone();

    let mut crate_batch = Vec::new();

    for _n in 0..instructions.crate_count {
        let from_crate_stack = updated_crate_stacks.get_mut(usize::from(instructions.from - 1)).unwrap();
        let from_crate_stack_last_element = from_crate_stack.pop().unwrap();

        crate_batch.push(from_crate_stack_last_element);
    }

    for (_index, crate_content) in crate_batch.iter().rev().enumerate() {
        let to_crate_stack = updated_crate_stacks.get_mut(usize::from(instructions.to - 1)).unwrap();
        to_crate_stack.push(crate_content.clone());
    }
    
    return updated_crate_stacks;
}

#[cfg(test)]
mod tests {
    use crate::_test_utils::setup;
    use super::*;

    fn _setup() -> String {
        read_input("src/day5/input.txt")
    }

    #[test]
    fn test_fill_crates_stack() {
        setup();
        assert_eq!(
            fill_crate_stacks(&"[T] [G] \n[S] [S] \n 1   2 ".to_string()),
            [
                ['S', 'T'],
                ['S', 'G'],
            ]
        );

        let mut vec = Vec::new();
        vec.push(vec!['S', 'T']);
        vec.push(vec!['G']);

        assert_eq!(
            fill_crate_stacks(&"[T]     \n[S] [G] \n 1   2 ".to_string()), vec
        );
    }

    #[test]
    fn test_move_stack() {
        setup();

        let mut initial_stack = Vec::new();
        initial_stack.push(vec!['S', 'T']);
        initial_stack.push(vec!['G']);

        let mut moved_stack = Vec::new();
        moved_stack.push(vec!['S']);
        moved_stack.push(vec!['G', 'T']);

        assert_eq!(
            move_stack(&mut initial_stack, Instructions {
                crate_count: 1,
                from: 1,
                to: 2,
            }),
            moved_stack
        );
    }
    
    #[test]
    fn test_move_stack_9001() {
        setup();

        let mut initial_stack = Vec::new();
        initial_stack.push(vec!['A', 'B', 'C']);
        initial_stack.push(vec!['D']);

        let mut moved_stack = Vec::new();
        moved_stack.push(vec!['A']);
        moved_stack.push(vec!['D', 'B', 'C']);

        assert_eq!(
            move_stack_9001(&mut initial_stack, Instructions {
                crate_count: 2,
                from: 1,
                to: 2,
            }),
            moved_stack
        );
    }
    
    #[test]
    fn test_read_instructions() {
        setup();

        let instructions = read_instructions(&"move 4 from 2 to 1".to_string());
        assert_eq!(instructions.crate_count, 4);
        assert_eq!(instructions.from, 2);
        assert_eq!(instructions.to, 1);
    }
}
