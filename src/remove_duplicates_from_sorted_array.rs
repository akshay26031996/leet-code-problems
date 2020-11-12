// https://leetcode.com/problems/remove-duplicates-from-sorted-array/

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 1 {
        return nums.len() as i32;
    }
    let mut j: usize = 0;
    for i in 0..nums.len() {
        if nums[j] != nums[i] {
            j += 1;
            nums[j] = nums[i];
        }
    }
    j += 1;
    let _ = nums.drain(j..);
    j as i32
}

#[cfg(test)]
mod tests {
    use super::remove_duplicates;
    #[test]
    fn remove_duplicates_from_sorted_array_basic() {
        let v1 = &mut vec![0, 0, 1, 1, 1, 2, 2, 3, 3, 4];
        let res = remove_duplicates(v1);
        assert_eq!(res, 5);
        assert_eq!(v1, &mut vec![0, 1, 2, 3, 4]);

        let v1 = &mut vec![1, 1, 2];
        let res = remove_duplicates(v1);
        assert_eq!(res, 2);
        assert_eq!(v1, &mut vec![1, 2]);
    }
}
