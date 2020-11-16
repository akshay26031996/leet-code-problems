// https://leetcode.com/problems/subarray-sum-equals-k/

use std::collections::HashMap;

pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut sum_store: HashMap<i32, i32> = HashMap::new();
    let mut current_sum = 0;
    let mut ans = 0;
    sum_store.insert(0, 1);
    for num in nums {
        current_sum += num;
        let desired_sum = current_sum - k;
        if let Some(v) = sum_store.get(&desired_sum) {
            ans += v;
        }
        match sum_store.get_mut(&current_sum) {
            None => {
                sum_store.insert(current_sum, 1);
            }
            Some(v) => {
                *v += 1;
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::subarray_sum;
    #[test]
    fn subarray_sum_equals_k_basic() {
        assert_eq!(subarray_sum(vec![1, 1, 1], 2), 2);
        assert_eq!(subarray_sum(vec![1, 2, 3], 3), 2);
    }
}
