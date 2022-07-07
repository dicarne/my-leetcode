pub struct Solution {}
mod sol2;
/// 暴力
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let mut d = dictionary.clone();
        d.sort_by(|a, b| a.len().cmp(&b.len()));
        let ss: Vec<&str> = sentence.split(" ").collect();
        let mut res: Vec<String> = vec![];
        for s in ss.iter() {
            let mut found = false;
            for ds in d.iter() {
                if s.starts_with(ds) {
                    found = true;
                    res.push(ds.clone());
                    break;
                }
            }
            if !found {
                res.push(s.to_string());
            }
        }
        return res.join(" ");
    }
}

#[cfg(test)]
mod tests {
    use crate::Solution;

    #[test]
    fn test1() {
        assert_eq!(
            "the cat was rat by the bat".to_string(),
            Solution::replace_words(
                vec!["cat".to_string(), "bat".to_string(), "rat".to_string()],
                "the cattle was rattled by the battery".to_string()
            )
        )
    }
}
