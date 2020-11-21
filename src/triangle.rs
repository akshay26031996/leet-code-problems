// https://leetcode.com/problems/triangle/

// use std::collections::HashMap;

// pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
//     fn minimum_total_util(
//         triangle: &Vec<Vec<i32>>,
//         row: usize,
//         col: usize,
//         calculated_answers: &mut HashMap<(usize, usize), i32>,
//     ) -> i32 {
//         // defining base conditions
//         if row >= triangle.len() {
//             return 0;
//         }
//         if col >= triangle.get(row).unwrap().len() {
//             return std::i32::MAX;
//         }
//         if let Some(ans) = calculated_answers.get(&(row, col)) {
//             return *ans;
//         }
//         let ans = triangle.get(row).unwrap().get(col).unwrap()
//             + std::cmp::min(
//                 minimum_total_util(&triangle, row + 1, col, calculated_answers),
//                 minimum_total_util(&triangle, row + 1, col + 1, calculated_answers),
//             );
//         calculated_answers.insert((row, col), ans);
//         ans
//     }
//     let mut calculated_answers: HashMap<(usize, usize), i32> = HashMap::new();
//     let ans = std::cmp::min(
//         minimum_total_util(&triangle, 0, 0, &mut calculated_answers),
//         minimum_total_util(&triangle, 0, 1, &mut calculated_answers),
//     );
//     ans
// }

pub fn minimum_total(triangle: Vec<Vec<i32>>) -> i32 {
    let mut dp = vec![0; triangle.len() + 1];
    for i in (0..triangle.len()).rev() {
        for j in 0..triangle.get(i).unwrap().len() {
            dp[j] = std::cmp::min(dp[j], dp[j + 1]) + triangle.get(i).unwrap().get(j).unwrap();
        }
    }
    dp[0]
}

#[cfg(test)]
mod tests {
    use super::minimum_total;
    #[test]
    fn triangle_basic() {
        assert_eq!(
            minimum_total(vec![vec![2], vec![3, 4], vec![6, 5, 7], vec![4, 1, 8, 3]]),
            11
        );
    }
}
