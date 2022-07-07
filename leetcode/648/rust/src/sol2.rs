use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub struct Solution {}

struct Node {
    child: Rc<RefCell<HashMap<char, Rc<RefCell<Node>>>>>,
}
/// 字典树
impl Solution {
    pub fn replace_words(dictionary: Vec<String>, sentence: String) -> String {
        let trie: Rc<RefCell<HashMap<char, Rc<RefCell<Node>>>>> =
            Rc::new(RefCell::new(HashMap::new()));
        for d in dictionary.iter() {
            let dw: Vec<char> = d.chars().collect();
            let mut cur = trie.clone();
            for c in dw.iter() {
                if !cur.borrow().contains_key(c) {
                    let a: Rc<RefCell<Node>> = Rc::new(RefCell::new(Node {
                        child: Rc::new(RefCell::new(HashMap::new())),
                    }));
                    cur.borrow_mut().insert(*c, a.clone());
                }
                let b = cur.borrow().get(c).unwrap().borrow_mut().child.clone();
                cur = b.clone();
            }
            cur.borrow_mut().insert(
                '#',
                Rc::new(RefCell::new(Node {
                    child: Rc::new(RefCell::new(HashMap::new())),
                })),
            );
        }

        let mut words: Vec<&str> = sentence.split(" ").collect();
        for i in 0..words.len() {
            let w = words[i];
            let word: Vec<char> = w.chars().collect();
            let mut cur = trie.clone();
            for j in 0..word.len() {
                let c = word[j];
                if cur.borrow().contains_key(&'#') {
                    words[i] = &w[..j];
                    break;
                }
                if !cur.borrow().contains_key(&c) {
                    break;
                }
                let b = cur.borrow().get(&c).unwrap().borrow().child.clone();
                cur = b.clone();
            }
        }

        return words.join(" ");
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

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
    #[test]
    fn test2() {
        assert_eq!(
            "a a a a a a a a bbb baba a".to_string(),
            Solution::replace_words(
                vec![
                    "a".to_string(),
                    "aa".to_string(),
                    "aaa".to_string(),
                    "aaaa".to_string()
                ],
                "a aa a aaaa aaa aaa aaa aaaaaa bbb baba ababa".to_string()
            )
        );
    }
}
