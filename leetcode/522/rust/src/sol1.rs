use super::base::Solution;
//


impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut str2 = strs.clone();
        str2.sort_by(|a, b| b.len().cmp(&(a.len())));
        let mut ans = -1;
        for i in 0..str2.len(){
            let mut nosub = true;
            for j in 0..str2.len() {
                if i != j && str2[i].len() <= str2[i].len() && Solution::is_sub(&str2[i], &str2[j]) {
                    nosub = false;
                    break;
                }
            }
            if nosub {
                ans = std::cmp::max(ans, str2[i].len() as i32);
            }
        }
        return ans
    }

    fn is_sub(a: &String, b: &String) -> bool {
        let mut il = 0;
        let mut ir = 0;
        let aa: Vec<char> = a.chars().collect();
        let bb: Vec<char> = b.chars().collect();
        while il < a.len() && ir < b.len() {
            if aa[il] == bb[ir] {
                il += 1;
            }
            ir += 1;
        }
        return il == aa.len()
    }
}