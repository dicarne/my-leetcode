pub struct Solution;


use std::collections::HashMap;
impl Solution {
    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        let word_len = words[0].len();
        let word_c = words.len();
        let total_len = words.len() * word_len;
        let mut all_words: HashMap<String, i32> = HashMap::new();
        for iw in words.iter() {
            let w = iw.clone();
            all_words.insert(w.clone(), all_words.get(&w).cloned().unwrap_or(0) + 1);
        }

        let mut res: Vec<i32> = Vec::new();
        for i in 0..s.len() - total_len + 1 {
            let mut tmp_words: HashMap<String, i32> = HashMap::new();
            for wi in 0..word_c {
                let w = &s[i + wi * word_len..i + (wi + 1) * word_len];
                if let Some(_) = all_words.get(&w.to_string()).cloned() {
                    tmp_words.insert(
                        w.to_string().clone(),
                        tmp_words.get(&w.to_string()).cloned().unwrap_or(0) + 1,
                    );
                }
            }
            if tmp_words == all_words {
                res.push(i as i32);
            }
        }
        return res;
    }
}