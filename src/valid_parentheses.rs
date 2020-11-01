// https://leetcode.com/problems/valid-parentheses/
use std::collections::HashMap;
pub fn is_valid(s: String) -> bool {
    let mut stack = Vec::new();
    let parentheses: HashMap<char, char> = [(')', '('), (']', '['), ('}', '{')]
        .iter()
        .cloned()
        .collect();
    let mut is_valid = true;
    for curr_char in s.chars() {
        if !parentheses.contains_key(&curr_char) {
            stack.push(curr_char);
        } else {
            match stack.pop() {
                None => {
                    is_valid = false;
                    break;
                }
                Some(prev_char) => {
                    if parentheses.get(&curr_char).unwrap() != &prev_char {
                        is_valid = false;
                        break;
                    }
                }
            }
        }
    }
    if !stack.is_empty() {
        return false;
    }
    is_valid
}

#[cfg(test)]
mod tests {
    use super::is_valid;
    #[test]
    fn valid_parentheses_basic() {
        assert_eq!(is_valid(String::from("{[]}")), true);
        assert_eq!(is_valid(String::from("{[}")), false);
    }
    #[test]
    fn valid_parentheses_non_empty_stack() {
        assert_eq!(is_valid(String::from("[")), false);
    }
}
