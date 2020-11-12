//  https://leetcode.com/problems/roman-to-integer/
use std::collections::HashMap;

pub fn roman_to_int(s: String) -> i32 {
    let roman_literal: HashMap<char, i32> = [
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .iter()
    .cloned()
    .collect();
    let mut res = 0;
    let iter = &mut s.chars().peekable();
    loop {
        let c = iter.next();
        if c.is_none() {
            break;
        }
        let c = c.unwrap();
        let c_val = roman_literal.get(&c).unwrap();
        if iter.peek().is_some() {
            eprintln!("{} : {}", c, iter.peek().unwrap());
            let n_val = roman_literal.get(&iter.peek().unwrap()).unwrap();
            if c_val < n_val {
                res -= c_val;
            } else {
                res += c_val;
            }
        } else {
            res += c_val;
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::roman_to_int;
    #[test]
    fn roman_to_integer_basic() {
        assert_eq!(roman_to_int("III".to_string()), 3);
        assert_eq!(roman_to_int("IV".to_string()), 4);
        assert_eq!(roman_to_int("LVIII".to_string()), 58);
        assert_eq!(roman_to_int("MCMXCIV".to_string()), 1994);
    }
}
