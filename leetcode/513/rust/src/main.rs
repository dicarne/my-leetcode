mod base;
use base::TreeNode;

use std::cell::RefCell;
use std::rc::Rc;
struct Solution {}

impl Solution {
    pub fn find_bottom_left_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let rootn = root.unwrap();
        let mut queue:std::collections::VecDeque<Rc<RefCell<TreeNode>>> = std::collections::VecDeque::new();
        queue.push_back(rootn.clone());
        let mut ans = 0;
        while !queue.is_empty() {
            let head = queue.pop_front();
            match head {
                Some(it) => {
                    let n = it.borrow();
                    match n.right.clone() {
                        Some(node) => queue.push_back(node),
                        None => ()
                    };
                    match n.left.clone() {
                        Some(node) => queue.push_back(node),
                        None => ()
                    };
                    ans = n.val;
                },
                None => break
            }
        }
        return ans;
    }
}

fn main() {
    let mut root = TreeNode::new(1);
    root.left = Some(Rc::new(RefCell::new(TreeNode::new(2))));
    root.right = Some(Rc::new(RefCell::new(TreeNode::new(3))));
    root.right.as_ref().unwrap().borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    //println!("{:?}", root);
    let ret = Solution::find_bottom_left_value(Some(Rc::new(RefCell::new(root))));
    if ret == 4 {
        println!("success");
    }else {
        println!("failed");
    }
}
