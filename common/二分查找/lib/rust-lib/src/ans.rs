extern crate rand;

use rand::Rng;

/// 生成随机的递增序列，数字不会重复
#[allow(dead_code)]
pub fn gen_random_unique_array() -> Vec<i32> {
    return u_gen_random_array(true);
}

/// 生成随机的递增序列，数字可能重复
#[allow(dead_code)]
pub fn gen_random_array() -> Vec<i32> {
    return u_gen_random_array(false);
}

fn u_gen_random_array(unique: bool) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let len = rng.gen_range(1..100);
    let mut array = vec![0 as i32; len];

    let mut last = -20;
    let low = if unique { 1 } else { 0 };
    for i in 0..len {
        let inc = rng.gen_range(low..(low + 5));
        array[i as usize] = last;
        last += inc;
    }
    return array;
}

pub fn gen_index(array: &Vec<i32>) -> usize {
    let mut rng = rand::thread_rng();
    return rng.gen_range(0..array.len());
}
