use super::base::Solution;

impl Solution {
    pub fn min_cost(costs: Vec<Vec<i32>>) -> i32 {
        let mut dp = costs[0].clone();
        let mut dpnew = vec![0,0,0];
        for i in 1..costs.len() {
            for j in 0..3 {
                dpnew[j] = std::cmp::min(dp[(j+1)%3], dp[(j+2)%3]) + costs[i][j];
            }
            dp = dpnew.clone();
        }
        return dp.iter().min().unwrap().clone();
    }
}