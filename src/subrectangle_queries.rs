// https://leetcode.com/problems/subrectangle-queries/

// Adding pub in order to remove the unused code warning.
// Don't want to add #[allow(dead_code)]

pub struct SubrectangleQueries {
    rectangle: Vec<Vec<i32>>,
}

impl SubrectangleQueries {
    pub fn new(rectangle: Vec<Vec<i32>>) -> Self {
        Self { rectangle }
    }

    pub fn update_subrectangle(
        &mut self,
        row1: i32,
        col1: i32,
        row2: i32,
        col2: i32,
        new_value: i32,
    ) {
        for i in row1..row2 + 1 {
            for j in col1..col2 + 1 {
                self.rectangle[i as usize][j as usize] = new_value;
            }
        }
    }

    pub fn get_value(&self, row: i32, col: i32) -> i32 {
        self.rectangle[row as usize][col as usize]
    }
}

/**
 * Your SubrectangleQueries object will be instantiated and called as such:
 * let obj = SubrectangleQueries::new(rectangle);
 * obj.update_subrectangle(row1, col1, row2, col2, newValue);
 * let ret_2: i32 = obj.get_value(row, col);
 */

#[cfg(test)]
mod tests {
    use super::SubrectangleQueries;
    #[test]
    fn subrectangle_queries_basic() {
        let mut rectangle = SubrectangleQueries::new(vec![
            vec![1, 2, 1],
            vec![4, 3, 4],
            vec![3, 2, 1],
            vec![1, 1, 1],
        ]);
        assert_eq!(rectangle.get_value(0, 2), 1);
        rectangle.update_subrectangle(0, 0, 3, 2, 5);
        assert_eq!(rectangle.get_value(0, 2), 5);
        assert_eq!(rectangle.get_value(3, 1), 5);
        rectangle.update_subrectangle(3, 0, 3, 2, 10);
        assert_eq!(rectangle.get_value(3, 1), 10);
        assert_eq!(rectangle.get_value(0, 2), 5);
    }
}
