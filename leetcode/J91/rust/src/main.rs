mod base;
mod sol1;
use base::Solution;

fn main() {
    judge_all();
}

fn judge_all () {
    judge(vec![vec![17,2,17], vec![16,16,5], vec![14,3,19]], 10).ok();
    judge(vec![vec![7,6,2]], 2).ok();
}

fn judge(costs: Vec<Vec<i32>>, ans: i32) -> Result<(), ()> {
    let res = Solution::min_cost(costs.clone());
    if res == ans {
        return Ok(())
    }
    println!("error!\ncosts: {:?}, \nans: {}, \nbut: {}", costs, ans, res);
    return Err(())
}
