// https://leetcode.com/problems/palindrome-number/
pub fn is_palindrome(x: i32) -> bool {
    if x.is_negative() {
        return false;
    }
    let x = x.to_string();
    x.char_indices()
        .zip(x.char_indices().rev())
        .take_while(|&((start_count, _), (end_count, _))| start_count < end_count)
        .all(|((_, start_char), (_, end_char))| start_char == end_char)
}
#[cfg(test)]
mod tests {
    use super::is_palindrome;
    #[test]
    fn palindrome_number_basic() {
        assert_eq!(is_palindrome(121), true);
        assert_eq!(is_palindrome(5), true);
        // -121 -> 121-
        assert_eq!(is_palindrome(-121), false);
        assert_eq!(is_palindrome(10), false);
    }
}
