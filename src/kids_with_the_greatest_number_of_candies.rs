// https://leetcode.com/problems/kids-with-the-greatest-number-of-candies/
pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
    let max_candies = candies.iter().max().unwrap();
    candies
        .iter()
        .map(|candies_count| candies_count + extra_candies >= *max_candies)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::kids_with_candies;
    #[test]
    fn kids_with_the_greatest_number_of_candies_basic() {
        assert_eq!(
            kids_with_candies(vec![2, 3, 5, 1, 3], 3),
            vec![true, true, true, false, true]
        );
        assert_eq!(
            kids_with_candies(vec![4, 2, 1, 1, 2], 1),
            vec![true, false, false, false, false]
        );
        assert_eq!(
            kids_with_candies(vec![12, 1, 12], 10),
            vec![true, false, true]
        );
    }
}
