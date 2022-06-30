use crate::base::Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let str1 = haystack.as_bytes();
        let str2 = needle.as_bytes();

        if needle.len() == 0 {
            return 0;
        }
        let next = Solution::gen_next(needle.as_str());
        println!("{:?}", next);
        let mut i = 0;
        let mut j: i32 = 0;
        while i < str1.len() && j < str2.len() as i32 {
            if j == -1 || str1[i] == str2[j as usize] {
                i += 1;
                j += 1;
            } else {
                j = next[j as usize];
            }
        }
        if j as usize == str2.len() {
            return (i as i32) - (j as i32);
        }
        return -1;
    }

    fn gen_next(str: &str) -> Vec<i32> {
        let astr = str.as_bytes();
        let mut next: Vec<i32> = vec![0; str.len()];
        next[0] = -1;
        let mut j: i32 = 0;
        let mut i = 1;
        while i < astr.len() - 1 {
            if j == -1 || astr[i] == astr[j as usize] {
                i += 1;
                j += 1;
                next[i] = j;
            } else {
                j = next[j as usize];
            }
        }
        return next;
    }
}
