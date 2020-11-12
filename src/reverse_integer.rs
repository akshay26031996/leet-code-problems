// https://leetcode.com/problems/reverse-integer/

pub fn reverse(x: i32) -> i32 {
    let mut res: i32 = 0;
    let mut input = x;
    if input == i32::MIN {
        return 0;
    }
    let is_neg = input.is_negative();
    if is_neg {
        input *= -1;
    }
    loop {
        if input == 0 {
            break;
        }
        let remainder = input % 10;
        input /= 10;
        let (tres, overflown) = res.overflowing_mul(10);
        if overflown {
            return 0;
        }
        res = tres;
        let (tres, overflown) = res.overflowing_add(remainder);
        if overflown {
            return 0;
        }
        res = tres;
    }
    if is_neg {
        let (tres, overflown) = res.overflowing_mul(-1);
        if overflown {
            return 0;
        }
        res = tres;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::reverse;
    #[test]
    fn reverse_integer_basic() {
        assert_eq!(reverse(32), 23);
    }
    #[test]
    fn reverse_integer_negative() {
        assert_eq!(reverse(-321), -123);
    }
    #[test]
    fn reverse_integer_overflow() {
        assert_eq!(reverse(2147483647), 0);
    }
}
