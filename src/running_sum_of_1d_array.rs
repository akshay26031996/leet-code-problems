// https://leetcode.com/problems/running-sum-of-1d-array/
pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
    let mut sum = 0;
    nums.iter()
        .map(|num| {
            sum += num;
            sum
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::running_sum;
    #[test]
    fn running_sum_of_1d_array_basic() {
        assert_eq!(running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    }
}
