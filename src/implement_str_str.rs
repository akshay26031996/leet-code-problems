// https://leetcode.com/problems/implement-strstr/

pub fn str_str(haystack: String, needle: String) -> i32 {
    let haystack_len = haystack.len();
    let needle_len = needle.len();
    if needle_len == 0 {
        return 0;
    }
    if needle_len > haystack_len {
        return -1;
    }
    let mut ans = -1;
    // forgot the +1 earler
    for i in 0..haystack_len - needle_len + 1 {
        if haystack[i..(i + needle_len)] == needle {
            ans = i as i32;
            break;
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::str_str;
    #[test]
    fn implement_str_str_basic() {
        assert_eq!(str_str(String::from("hello"), String::from("ll")), 2);
        assert_eq!(str_str(String::from("aaaaa"), String::from("bba")), -1);
        assert_eq!(str_str(String::from(""), String::from("")), 0);
        assert_eq!(str_str(String::from("a"), String::from("a")), 0);
    }
}
