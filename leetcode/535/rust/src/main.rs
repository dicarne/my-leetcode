mod sol1;
use sol1::Codec;

fn main() {
    judge("https://leetcode.com/problems/design-tinyurl".to_string());
}

fn judge(input: String) {
    let mut obj = Codec::new();
    let encode_res: String = obj.encode(input.clone());
    let decode_res: String = obj.decode(encode_res.clone());
    if decode_res == input {
        
    } else {
        println!("error:");
        println!("{}", encode_res);
        println!("{}", decode_res);
    }
}
