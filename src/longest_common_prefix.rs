// https://leetcode.com/problems/longest-common-prefix/
use std::str::Chars;

pub fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut res = String::new();
    if strs.is_empty() {
        return res;
    }
    let mut iters: Vec<Chars> = strs.iter().map(|str| str.chars()).collect();
    let mut is_complete = false;
    loop {
        let mut loop_char: Option<char> = None;
        for iter in &mut iters {
            let curr_char: Option<char> = iter.next();
            match curr_char {
                None => {
                    is_complete = true;
                    break;
                }
                Some(c) => {
                    if loop_char.is_none() {
                        loop_char = Some(c);
                    }
                    if loop_char != curr_char {
                        is_complete = true;
                        break;
                    }
                }
            }
        }
        if is_complete {
            break;
        }
        res.push(loop_char.unwrap());
    }
    res
}
#[cfg(test)]
mod tests {
    use super::longest_common_prefix;
    #[test]
    fn longest_common_prefix_basic() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        );
    }
    #[test]
    fn longest_common_prefix_empty() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("dog"),
                String::from("racecar"),
                String::from("car")
            ]),
            String::from("")
        );
    }
}
