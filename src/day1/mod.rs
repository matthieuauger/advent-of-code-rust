use std::fs;

pub fn main() {
    part2();
}

fn _part1() {
    let file_contents = fs::read_to_string("src/day1/input.txt")
        .expect("Error reading file");

    let mut max_total_calories = 0;
    let mut total_calories = 0;
    for line in file_contents.lines() {
        if line.is_empty() {
            if total_calories > max_total_calories {
                max_total_calories = total_calories;
            }

            total_calories = 0;
            continue;
        }

        total_calories += line.parse::<i32>().unwrap();
    }

    println!("Max total calories: {}", max_total_calories);
}

fn part2() {
    let file_contents = fs::read_to_string("src/day1/input.txt")
        .expect("Error reading file");

    let mut max_total_calories = [0, 0, 0];

    let mut total_calories = 0;
    for line in file_contents.lines() {
        if line.is_empty() {
            if total_calories > max_total_calories[0] {
                max_total_calories[2] = max_total_calories[1];
                max_total_calories[1] = max_total_calories[0];
                max_total_calories[0] = total_calories;
            } else if total_calories > max_total_calories[1] {
                max_total_calories[2] = max_total_calories[1];
                max_total_calories[1] = total_calories;
            } else if total_calories > max_total_calories[2] {
                max_total_calories[2] = total_calories;
            }

            total_calories = 0;
            continue;
        }

        total_calories += line.parse::<i32>().unwrap();
    }

    println!("Max total calories: {}", max_total_calories.iter().sum::<i32>());
}
