pub struct Solution {}

impl Solution {
    pub fn min_cost_to_move_chips(position: Vec<i32>) -> i32 {
        let mut even = 0;
        let mut odd = 0;
        for i in position.iter() {
            if i % 2 == 0 {
                even += 1;
            } else {
                odd += 1;
            }
        }
        return std::cmp::min(even, odd);
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn it_works() {
        assert_eq!(1, Solution::min_cost_to_move_chips(vec![1, 2, 3]));
    }
}
