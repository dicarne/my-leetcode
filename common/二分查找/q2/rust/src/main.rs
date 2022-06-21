extern crate ans_lib;
use ans_lib::ans;
extern crate rand;
use rand::Rng;

fn main() {
    match judge_many() {
        Ok(_) => println!("success!"),
        Err(_) => println!("failed!"),
    };
}

fn judge_many() -> Result<(), ()> {
    for _ in 0..100 {
        judge()?;
    }
    return Ok(());
}

fn judge() -> Result<(), ()> {
    let mut rng = rand::thread_rng();
    let array = ans::gen_random_array();
    let value = rng.gen_range(array[0] - 10..array[array.len() - 1] + 10);
    let mut index = -1 as i32;
    for i in 0..array.len() - 1 {
        if array[i] <= value && array[i + 1] > value {
            index = (i + 1) as i32;
        }
    }
    if index == -1 {
        if value < array[0] {
            index = 0;
        } else if value >= array[array.len() - 1] {
            index = array.len() as i32;
        } else {
            return Err(());
        }
    }

    match find_ge(&array, value) {
        Ok(ind) => {
            if ind == index as usize {
                return Ok(());
            } else {
                println!("v:{}, i:{}, arr:{:?}", value, index, array);
                return Err(());
            }
        }
        Err(_) => {
            println!("Error: #{}, {}# {:?}", value, index, array);
            return Err(());
        }
    }
}

fn find_ge(array: &Vec<i32>, value: i32) -> Result<usize, ()> {
    let mut index = find_n(array, value)?;
    let len = array.len();
    // 如果找到的值和value相等，则找到第一个大于它的位置
    while index < len && value == array[index] {
        index += 1;
    }
    return Ok(index);
}

/// 模糊查找某个值所在的位置
fn find_n(array: &Vec<i32>, value: i32) -> Result<usize, ()> {
    if array.len() == 0 {
        return Err(());
    }
    let mut left = 0;
    let mut right = (array.len() - 1) as i32;

    // 一定要 <=
    while left <= right {
        let mid = (left + right) / 2;
        if array[mid as usize] == value {
            return Ok(mid as usize);
        } else if value < array[mid as usize] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return Ok(left as usize);
}
