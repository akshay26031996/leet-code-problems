// https://leetcode.com/problems/rectangle-overlap/

pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
    if rec1.get(0).unwrap() == rec1.get(2).unwrap()
        || rec1.get(1).unwrap() == rec1.get(3).unwrap()
        || rec2.get(0).unwrap() == rec2.get(2).unwrap()
        || rec2.get(1).unwrap() == rec2.get(3).unwrap()
    {
        return false;
    }
    if ((rec1.get(0).unwrap() >= rec2.get(0).unwrap()
        && rec1.get(0).unwrap() < rec2.get(2).unwrap())
        || (rec2.get(0).unwrap() >= rec1.get(0).unwrap()
            && rec2.get(0).unwrap() < rec1.get(2).unwrap()))
        && ((rec1.get(1).unwrap() >= rec2.get(1).unwrap()
            && rec1.get(1).unwrap() < rec2.get(3).unwrap())
            || (rec2.get(1).unwrap() >= rec1.get(1).unwrap()
                && rec2.get(1).unwrap() < rec1.get(3).unwrap()))
    {
        return true;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::is_rectangle_overlap;
    #[test]
    fn rectangle_overlap_basic() {
        assert_eq!(
            is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
            true
        );
        assert_eq!(
            is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
            false
        );
        assert_eq!(
            is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]),
            false
        );
    }
}
