mod base;
mod sol1;

fn main() {
}

#[cfg(test)]
mod tests {
    use crate::base::Solution;

    #[test]
    fn all() {
        assert_eq!(Solution::minimum_abs_difference(vec![4,2,1,3]), vec![vec![1,2],vec![2,3],vec![3,4]]);
        assert_eq!(Solution::minimum_abs_difference(vec![1,3,6,10,15]), vec![vec![1,3]]);
        assert_eq!(Solution::minimum_abs_difference(vec![3,8,-10,23,19,-4,-14,27]), vec![[-14,-10],[19,23],[23,27]]);
    }
}
