// https://leetcode.com/problems/reverse-string/

pub fn reverse_string(s: &mut Vec<char>) {
    let length = s.len();
    for i in 0..(length / 2) {
        s.swap(i, length - 1 - i);
    }
}

#[cfg(test)]
mod tests {
    use super::reverse_string;
    #[test]
    fn reverse_string_odd() {
        let mut s = vec!['h', 'e', 'l', 'l', 'o'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['o', 'l', 'l', 'e', 'h']);
    }
    #[test]
    fn reverse_string_even() {
        let mut s = vec!['H', 'a', 'n', 'n', 'a', 'h'];
        reverse_string(&mut s);
        assert_eq!(s, vec!['h', 'a', 'n', 'n', 'a', 'H']);
    }
}
