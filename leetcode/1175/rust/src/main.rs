use base::Solution;

mod base;
mod sol1;


fn main() {
    judge_all();
}

fn judge_all() {
    judge(5, 12).unwrap();

    judge(100, 682289015).unwrap();
}

fn judge(a: i32, b: i32) -> Result<(),()> {
    let ans = Solution::num_prime_arrangements(a);
    if ans == b {
        Ok(())
    } else {
        println!("{} {} {}", a, b, ans);
        Err(())
    }
}