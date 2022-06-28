mod base;
mod sol1;
use std::borrow::BorrowMut;

use base::Solution;

fn main() {
    match judge(vec![1,5,1,1,6,4]) {
        Err((a, b)) => {
            println!("err: {:?}\nbut: {:?}", a, b);
        }
        Ok(_) => {}
    }
}

fn judge(nums: Vec<i32>) -> Result<(), (Vec<i32>, Vec<i32>)> {
    let mut arr = nums.clone();
    Solution::wiggle_sort(arr.borrow_mut());
    for i in 1..arr.len() {
        if i % 2 == 1 {
            if arr[i - 1] > arr[i] {
                return Err((nums, arr));
            }
        } else {
            if arr[i - 1] < arr[i] {
                return Err((nums, arr));
            }
        }
    }
    return Ok(());
}
