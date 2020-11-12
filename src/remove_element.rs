// https://leetcode.com/problems/remove-element/

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut j: usize = 0;
    for i in 0..nums.len() {
        nums[j] = nums[i];
        if nums[j] != val {
            j += 1;
        }
    }
    let _ = nums.drain(j..);
    j as i32
}

#[cfg(test)]
mod tests {
    use super::remove_element;
    #[test]
    fn remove_element_basic() {
        let v1 = &mut vec![3, 2, 2, 3];
        let res = remove_element(v1, 3);
        assert_eq!(res, 2);
        assert_eq!(v1, &mut vec![2, 2]);

        let v1 = &mut vec![0, 1, 2, 2, 3, 0, 4, 2];
        let res = remove_element(v1, 2);
        assert_eq!(res, 5);
        assert_eq!(v1, &mut vec![0, 1, 3, 0, 4]);
    }
}
