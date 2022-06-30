use crate::base::Solution;

impl Solution {
    pub fn num_prime_arrangements(n: i32) -> i32 {
        if n < 2 {
            return 1;
        }
        let mut p = 0;
        let mut sum:i64 = 1;
        let m = 1000000007;
        for i in 2..n + 1 {
            if Solution::is_prime(i) {
                p += 1;
            }
        }
        for i in 2..p+1 {
            sum = (sum * i) % m;
        }
        let n2 = n as i64;
        for i in 2..(n2-p+1) {
            sum = (sum * i) % m;
        }
        return sum as i32;
    }

    fn is_prime(n: i32) -> bool {
        if n < 2 {
            return true;
        }
        for i in 2..n {
            if n % i == 0 {
                return false;
            }
        }
        return true;
    }
}
