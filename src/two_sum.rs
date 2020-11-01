// https://leetcode.com/problems/two-sum/
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut res = Vec::new();
    let len = nums.len();
    for i in 0..len {
        for j in i + 1..len {
            if nums[i] + nums[j] == target {
                res.push(i as i32);
                res.push(j as i32);
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::two_sum;
    #[test]
    fn two_sum_basic() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![0, 1]);
    }
}
