use crate::base::Solution;

impl Solution {
    pub fn minimum_abs_difference(arr: Vec<i32>) -> Vec<Vec<i32>> {
        let mut sarr = arr.clone();
        sarr.sort();
        let mut mmin = 10000000;
        let mut res: Vec<Vec<i32>> = vec![];

        for i in 0..sarr.len() - 1 {
            let mabs = sarr[i + 1] - sarr[i];
            if mabs < mmin {
                mmin = mabs;
                res.clear()
            }
            if mabs == mmin {
                res.push(vec![sarr[i], sarr[i + 1]]);
            }
            
        }
        return res;
    }
}

