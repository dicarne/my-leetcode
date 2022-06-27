mod base;
mod sol1;
use base::Solution;

fn main() {
    judge_all().ok();
}

fn judge_all() -> Result<(),()>{
    judge(vec!["aba".to_string(), "cdc".to_string(), "eae".to_string()], 3)?;
    judge(vec!["aaa".to_string(), "aaa".to_string(), "aa".to_string()], -1)?;
    Ok(())
}

fn judge(arr: Vec<String>, ans: i32) -> Result<(), ()> {
    let a = Solution::find_lu_slength(arr.clone());
    if a == ans {
        return Ok(());
    }
    println!("arr: {:?}\nans: {:?}\nbut: {:?}\n", arr, ans, a);
    return Err(());
}
