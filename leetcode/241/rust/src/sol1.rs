use crate::base::Solution;

impl Solution {
    pub fn diff_ways_to_compute(expression: String) -> Vec<i32> {
        let vcstr: Vec<char> = expression.chars().collect();
        return Solution::_diff_ways_to_compute(&vcstr[..]);
    }

    fn _diff_ways_to_compute(expr: &[char]) -> Vec<i32> {
        let mut res: Vec<i32> = vec![];
        let mut i = 0;
        while i < expr.len() {
            if expr[i] == '+' || expr[i] == '-' || expr[i] == '*' {
                let preans = Solution::_diff_ways_to_compute(&expr[..i]);
                let remain = Solution::_diff_ways_to_compute(&expr[i + 1..]);
                for p in preans.iter() {
                    let pa = p.clone();
                    for j in remain.iter() {
                        let rans = j.clone();
                        match expr[i] {
                            '+' => res.push(pa + rans),
                            '-' => res.push(pa - rans),
                            '*' => res.push(pa * rans),
                            _ => {}
                        }
                    }
                }
            }
            i += 1;
        }
        if res.is_empty() {
            res.push(Solution::to_num(expr));
        }
        return res;
    }

    fn to_num(arr: &[char]) -> i32 {
        let mut sum = 0;

        for i in 0..arr.len() {
            let ind = arr.len() - i - 1;
            sum += ((arr[i] as i32) - ('0' as i32)) * (10_i32.pow((ind) as u32));
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use crate::base::Solution;

    #[test]
    fn test_to_num() {
        assert_eq!(1, Solution::to_num(&['1']));
        assert_eq!(10, Solution::to_num(&['1', '0']));
        assert_eq!(0, Solution::to_num(&['0']));
        assert_eq!(53, Solution::to_num(&['5', '3']));
    }
}
