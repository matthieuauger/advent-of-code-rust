use std::fs;
use std::collections::HashSet;

pub fn read_input(file_path: &str) -> String {
  let file_contents = fs::read_to_string(file_path)
  .expect("Error reading file");

  return file_contents;
}

pub fn split_string_middle(string: String) -> (String, String) {
  let (string_1, string_2) = string.split_at(string.len() / 2);
  
  return (string_1.to_string(), string_2.to_string());
}

pub fn find_common_item(string_1: String, string_2: String) -> char {
  let string_1_chars: HashSet<char> = string_1.chars().collect();
  let string_2_chars: HashSet<char> = string_2.chars().collect();
  
  let common_chars: HashSet<char> = string_1_chars.intersection(&string_2_chars).cloned().collect();

  return *common_chars.iter().next().unwrap();
}

pub fn find_common_item_in_three_strings(string_1: String, string_2: String, string_3: String) -> char {
  let string_1_chars: HashSet<char> = string_1.chars().collect();
  let string_2_chars: HashSet<char> = string_2.chars().collect();
  let string_3_chars: HashSet<char> = string_3.chars().collect();
  
  let common_chars: HashSet<char> = string_1_chars.intersection(&string_2_chars).cloned().collect();
  let common_chars: HashSet<char> = common_chars.intersection(&string_3_chars).cloned().collect();

  return *common_chars.iter().next().unwrap();
}
