// https://leetcode.com/problems/search-insert-position/
pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
    // Binary Search
    let mut start = 0;
    let mut end = nums.len() - 1;
    if nums[end] < target {
        return end as i32 + 1;
    }
    if nums[start] > target {
        return 0;
    }
    let mut middle;
    while start < end {
        middle = start + ((end - start) / 2);
        match nums[middle].cmp(&target) {
            std::cmp::Ordering::Equal => {
                start = middle;
                end = middle;
            }
            std::cmp::Ordering::Less => {
                start = middle + 1;
            }
            std::cmp::Ordering::Greater => {
                end = match middle.checked_sub(1) {
                    None => 0,
                    Some(middle) => middle,
                };
            }
        }
    }

    // eprintln!("Current value from binary search is {}", start);
    let next_index = start + 1;
    if nums[start] == target {
        start as i32
    } else if nums[start] < target && nums[next_index] > target {
        next_index as i32
    } else {
        start as i32
    }
}
#[cfg(test)]
mod tests {
    use super::search_insert;
    #[test]
    fn search_insert_position_basic() {
        assert_eq!(search_insert(vec![1, 3, 5, 6], 5), 2);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 7), 4);
        assert_eq!(search_insert(vec![1, 3, 5, 6], 0), 0);
        assert_eq!(search_insert(vec![1], 0), 0);
    }
    #[test]
    fn search_insert_position_regression() {
        assert_eq!(search_insert(vec![1, 3], 0), 0);
        assert_eq!(search_insert(vec![1, 3], 2), 1);
        assert_eq!(search_insert(vec![1, 3, 5], 1), 0);
    }
}
