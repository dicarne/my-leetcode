mod sol1;

fn main() {
    let a = sol1::Solution::evaluate("(let x 2 (mult x (let x 3 y 4 (add x y))))".to_string());
}
