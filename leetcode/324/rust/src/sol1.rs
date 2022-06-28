
use super::base::Solution;

impl Solution {
    pub fn wiggle_sort(nums: &mut Vec<i32>) {
        nums.sort();
        let center = if nums.len() % 2 == 0 {
            nums.len() / 2
        } else {
            nums.len() / 2 + 1
        };
        let mut left: Vec<i32> = Vec::with_capacity(center);
        left.resize(center, 0);
        let mut right: Vec<i32> = Vec::with_capacity(nums.len() - center);
        right.resize(nums.len() - center, 0);
        left.clone_from_slice(&nums[0..center]);
        right.clone_from_slice(&nums[center..]);
        left.reverse();
        right.reverse();

        let mut li = 0;
        let mut ri = 0;
        for i in 0..nums.len() {
            if i % 2 == 0 {
                nums[i] = left[li];
                li += 1;
            } else {
                nums[i] = right[ri];
                ri += 1;
            }
        }
    }
}
