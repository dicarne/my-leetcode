mod base;
mod sol1;
use base::Solution;

fn main() {
    let mut res = Solution::diff_ways_to_compute("2-1-1".to_string());
    res.sort();
    assert_eq!(res, vec![0, 2]);
}
