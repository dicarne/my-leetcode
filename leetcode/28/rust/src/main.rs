mod base;
mod sol1;
use base::Solution;

fn main() {
    judge("aabaaabaaac".to_string(), "aabaaac".to_string(), 4);
    judge("ababababca".to_string(), "abababca".to_string(), 2);
    judge("hello".to_string(), "ll".to_string(), 2);
}

fn judge(a: String, b: String, ans: i32) {
    let s = Solution::str_str(a, b);
    assert_eq!(s, ans);
}
