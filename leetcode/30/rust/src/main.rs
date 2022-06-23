

mod sol1;
use sol1::Solution;

fn main() {
    match judge_all() {
        Ok(_) => println!("success!"),
        Err(_) => println!("failed!"),
    }
}

fn judge_all() -> Result<(), ()> {
    judge("barfoothefoobarman", vec!["foo", "bar"], vec![0, 9])?;
    judge(
        "wordgoodgoodgoodbestword",
        vec!["word", "good", "best", "word"],
        vec![],
    )?;
    judge(
        "barfoofoobarthefoobarman",
        vec!["bar", "foo", "the"],
        vec![6, 9, 12],
    )?;
    return Ok(());
}

fn judge(s: &str, words: Vec<&str>, result: Vec<i32>) -> Result<(), ()> {
    let sa = s.clone();
    let sw = words.clone();
    let mut qes_v: Vec<String> = Vec::new();
    for i in words.iter() {
        qes_v.push(i.to_string());
    }
    let res = Solution::find_substring(s.to_string(), qes_v);
    if !same_vec(&res, &result) {
        println!(
            "Err: str[{}], words: {:?}, \nresult: {:?}, \nhowever: {:?}",
            sa, sw, result, res
        );
        return Err(());
    }
    return Ok(());
}

fn same_vec(a1: &Vec<i32>, a2: &Vec<i32>) -> bool {
    if a1.len() != a2.len() {
        return false;
    }
    let mut ia1 = a1.clone();
    ia1.sort();
    let mut ia2 = a2.clone();
    ia2.sort();
    return a1 == a2;
}
