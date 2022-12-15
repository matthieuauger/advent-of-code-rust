use crate::_utils::read_input;

#[allow(dead_code)]
pub fn main() {
    let file_contents = read_input("src/day6/input.txt");

    part1(&file_contents);

}

fn part1(_file_contents: &String) -> u32 {

    return 0;
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
