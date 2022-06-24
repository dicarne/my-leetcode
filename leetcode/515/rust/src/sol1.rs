use crate::base;
use base::Solution;
use base::TreeNode;

use std::rc::Rc;
use std::cell::RefCell;
use std::collections::VecDeque;

impl Solution {
    pub fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        if let Some(node) = root {
            let mut maxvalues: Vec<i32> = vec![];
            let mut queue: VecDeque<Rc<RefCell<TreeNode>>> = VecDeque::from([node]);
            while !queue.is_empty() {
                let mut q: VecDeque<Rc<RefCell<TreeNode>>>  = VecDeque::new();
                let mut maxv: Option<i32> = None;
                while !queue.is_empty() {
                    if let Some(n) = queue.pop_front() {
                        match maxv {
                            None => maxv = Some(n.borrow().val),
                            Some(oldv) => {
                                let v = n.borrow().val;
                                if oldv < v {
                                    maxv = Some(v);
                                }
                            }
                        }
                        if let Some(nn) = n.borrow().left.clone() {
                            q.push_back(nn);
                        }
                        if let Some(nn) = n.borrow().right.clone() {
                            q.push_back(nn);
                        }
                    }
                }
                if let Some(v) = maxv {
                    maxvalues.push(v);
                }
                queue = q;
            }
            return maxvalues
        }
        
        return vec![]
    }
}