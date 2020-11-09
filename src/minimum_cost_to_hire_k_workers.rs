// https://leetcode.com/problems/minimum-cost-to-hire-k-workers/

use std::collections::BinaryHeap;
pub fn mincost_to_hire_workers(quality: Vec<i32>, wage: Vec<i32>, k: i32) -> f64 {
    if k == 0 {
        return 0.0;
    }
    let k = k as usize;
    let mut worker_info: Vec<(f64, i32, i32)> = wage
        .iter()
        .zip(quality.iter())
        .map(|(&w, &q)| (w as f64 / q as f64, q, w))
        .collect();
    // sort the workers from lowest to highest (wage to quality) ratio
    worker_info.sort_by(|(first, _, _), (second, _, _)| first.partial_cmp(second).unwrap());
    let mut ans: f64 = std::f64::MAX;
    let mut sum = 0;
    let mut heap = BinaryHeap::<i32>::new();
    // start putting workers in the heap and as soon as you have `k`
    // workers calculate the cost using the ratio of kth worker
    // if you have more than k worker, remove the worker with maximum
    // quality till now since cost will be directly proportional to that
    for (r, q, _) in worker_info {
        heap.push(q);
        sum += q;
        if heap.len() > k {
            sum -= heap.pop().unwrap();
        }
        if heap.len() == k {
            let x = sum as f64 * r;
            if x < ans {
                ans = x
            }
        }
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::mincost_to_hire_workers;

    #[test]
    fn minimum_cost_to_hire_k_workers_basic() {
        assert_eq!(
            (mincost_to_hire_workers(vec![10, 20, 5], vec![70, 50, 30], 2) - 105.0).abs() < 1.0e-5,
            true
        );
        assert_eq!(
            (mincost_to_hire_workers(vec![3, 1, 10, 10, 1], vec![4, 8, 2, 2, 7], 3) - 30.66667)
                .abs()
                < 1.0e-5,
            true
        );
    }
}
