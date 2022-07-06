pub struct Solution {}

use std::{cell::RefCell, collections::HashMap, rc::Rc};

struct Enode {
    values: HashMap<String, i32>,
    ret: i32,
    inc: usize,
    parent: Option<Rc<RefCell<Enode>>>,
}

impl Solution {
    pub fn evaluate(expression: String) -> i32 {
        let ae: Vec<char> = expression.chars().collect();
        let root = Enode {
            values: HashMap::new(),
            ret: 0,
            inc: 0,
            parent: None,
        };
        let rootn = Rc::new(RefCell::new(root));
        let r = Solution::calc(rootn, &ae[..]);
        return r.borrow().ret;
    }

    /// 用于计算下一个表达式，如果是函数，则将整个函数计算完毕，如果是值，则直接返回值。
    fn calc(parent: Rc<RefCell<Enode>>, expr: &[char]) -> Rc<RefCell<Enode>> {
        let (head, inc) = Solution::get_token(expr);
        let nc = Rc::new(RefCell::new(Enode {
            values: HashMap::new(),
            ret: 0,
            inc,
            parent: Some(parent),
        }));
        if head[0] == '(' {
            let ntk = &head[1..].iter().collect::<String>();
            match ntk.as_str() {
                "let" => {
                    loop {
                        // 第奇数个，可能是赋值的变量标识符，也可能是最后一个
                        let (k, skip) = Solution::get_token(&expr[nc.borrow().inc..]);

                        if k[0] == '(' {
                            // 最后的expr，但是是个表达式
                            let kv = Solution::calc(nc.clone(), &expr[nc.borrow().inc..]);
                            let kvv = kv.borrow();
                            // 之所以每次都要nc.borrow_mut()，是因为在递归的访问parent时，会出现同时存在可变借用和不可变借用的情况。因此尽量早的释放可变借用。
                            nc.borrow_mut().inc += kvv.inc;
                            nc.borrow_mut().ret = kvv.ret;
                            break;
                        } else if k[k.len() - 1] == ')' {
                            // 最后的expr，但是是个量
                            nc.borrow_mut().inc += k.len();
                            nc.borrow_mut().ret = Solution::get_value(
                                k[..k.len() - 1].iter().collect::<String>(),
                                nc.clone(),
                            );
                            break;
                        } else {
                            // 中间的量，一定是值或者表达式
                            nc.borrow_mut().inc += skip;
                            let vp = Solution::calc(nc.clone(), &expr[nc.borrow().inc..]);
                            let v = vp.borrow();
                            nc.borrow_mut().inc += v.inc;
                            nc.borrow_mut()
                                .values
                                .insert(k.iter().collect::<String>(), v.ret);
                        }
                    }
                }
                "add" => {
                    let a = Solution::calc(nc.clone(), &expr[nc.borrow().inc..]);
                    let an = a.borrow_mut();
                    nc.borrow_mut().inc += an.inc;
                    let b = Solution::calc(nc.clone(), &expr[nc.borrow().inc..]);
                    let bn = b.borrow_mut();
                    nc.borrow_mut().inc += bn.inc;
                    nc.borrow_mut().ret = an.ret + bn.ret;
                }
                "mult" => {
                    let a = Solution::calc(nc.clone(), &expr[nc.borrow().inc..]);
                    let an = a.borrow_mut();
                    nc.borrow_mut().inc += an.inc;
                    let b = Solution::calc(nc.clone(), &expr[nc.borrow().inc..]);
                    let bn = b.borrow_mut();
                    nc.borrow_mut().inc += bn.inc;
                    nc.borrow_mut().ret = an.ret * bn.ret;
                }
                _ => {
                    panic!("unknown function!: {}", ntk);
                }
            }

            return nc.clone();
        } else {
            let mut headstr = head;
            if head[head.len() - 1] == ')' {
                headstr = &head[..head.len() - 1];
            }
            let str = headstr.iter().collect::<String>();
            nc.borrow_mut().ret = Solution::get_value(str, nc.clone());
            return nc.clone();
        }
    }

    /// 获取下一个token，token只可能是：数字字符串，标识符，带前括号的标识符，带后括号的标识符。
    fn get_token(expr: &[char]) -> (&[char], usize) {
        let mut i = 0;
        while i < expr.len() {
            if expr[i] == ' ' || expr[i] == ')' {
                i += 1;
            } else {
                break;
            }
        }
        let beg = i;
        let mut end = i;
        let mut meet_right = false;
        while i < expr.len() {
            if expr[i] != ' ' {
                if expr[i] != ')' {
                    end += 1;
                } else {
                    if !meet_right {
                        end += 1;
                        meet_right = true;
                    }
                }
                i += 1;
            } else {
                break;
            }
        }
        return (&expr[beg..end], i);
    }

    /// 根据字符串获取值：如果是数字，则转换成数字；如果是标识符，则从子级到父级查找值。
    fn get_value(token: String, ctx: Rc<RefCell<Enode>>) -> i32 {
        let try_num = token.parse::<i32>();
        match try_num {
            Ok(v) => return v,
            Err(_) => {}
        }
        let n = ctx.borrow();
        match n.values.get(&token) {
            Some(v) => return v.clone(),
            None => match &n.parent {
                Some(p) => {
                    return Solution::get_value(token, p.clone());
                }
                None => {
                    panic!("no parent!!");
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn runall() {
        assert_eq!(
            14,
            Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string())
        );
        assert_eq!(2, Solution::evaluate("(let x 3 x 2 x)".to_string()));
        assert_eq!(
            5,
            Solution::evaluate("(let x 1 y 2 x (add x y) (add x y))".to_string())
        );
        assert_eq!(
            6,
            Solution::evaluate("(let x 2 (add (let x 3 (let x 4 x)) x))".to_string())
        );
    }
}
