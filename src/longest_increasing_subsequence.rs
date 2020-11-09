// https://leetcode.com/problems/longest-increasing-subsequence/
// follow up - do it in nlogn

use std::cmp::max;

pub fn length_of_lis(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }
    let mut ans = 1;
    let mut dp = vec![1; nums.len()];
    for i in 1..nums.len() {
        for j in 0..i {
            if nums[i] > nums[j] {
                dp[i] = max(dp[j] + 1, dp[i]);
            }
        }
        if dp[i] > ans {
            ans = dp[i];
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::length_of_lis;
    #[test]
    fn longest_increasing_subsequence_basic() {
        assert_eq!(length_of_lis(vec![10, 9, 2, 5, 3, 7, 101, 18]), 4);
    }
    #[test]
    fn longest_increasing_subsequence_empty() {
        assert_eq!(length_of_lis(vec![]), 0);
    }
    #[test]
    fn longest_increasing_subsequence_single() {
        assert_eq!(length_of_lis(vec![0]), 1);
    }
}
