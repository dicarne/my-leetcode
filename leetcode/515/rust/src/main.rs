use std::{cell::RefCell, collections::VecDeque, rc::Rc};

use base::TreeNode;
use base::Solution;

mod base;
mod sol1;

fn main() {
    let root = build_tree(vec![Some(1),Some(3),Some(2),Some(5),Some(3),None,Some(9)]);
    let result = Solution::largest_values(root);
    assert_eq!(result, vec![1,3,9]);

}

fn build_tree(tr: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let root = make_node(tr[0].unwrap());
    let mut queue = VecDeque::from([root.clone()]);
    let mut numq = VecDeque::from(tr);
    numq.pop_front();
    while !queue.is_empty() {
        if let Some(head) = queue.pop_front() {
            if let Some(v) = numq.pop_front() {
                if let Some(vv) = v {
                    let node =  make_node(vv);
                    head.clone().unwrap().borrow_mut().left = node.clone();
                    queue.push_back(node);
                }
            }
            if let Some(v) = numq.pop_front() {
                if let Some(vv) = v {
                    let node =  make_node(vv);
                    head.clone().unwrap().borrow_mut().right = node.clone();
                    queue.push_back(node);
                }
            }
        }
    }
    return root;
}

fn make_node(val: i32) -> Option<Rc<RefCell<TreeNode>>> {
    Some(Rc::new(RefCell::new(TreeNode::new(val))))
}
