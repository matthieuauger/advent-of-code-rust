use crate::_utils::read_input;

#[allow(dead_code)]
pub fn main() {
    let file_contents = read_input("src/day1/input.txt");

    let max_total_calories = part1(&file_contents);
    println!("Max total calories: {}", max_total_calories);

    let max_total_calories = part2(&file_contents);
    println!("Max total calories: {}", max_total_calories);
}

fn part1(file_contents: &String) -> i32 {
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

    return max_total_calories;
}

fn part2(file_contents: &String) -> i32 {
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

    return max_total_calories.iter().sum::<i32>();
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> String {
        return read_input("src/day1/input.txt");
    }

    #[test]
    fn test_part1() {
        let input = setup();

        assert_eq!(part1(&input), 68775);
    }

    #[test]
    fn test_part2() {
        let input = setup();

        assert_eq!(part2(&input), 202585);
    }
}
