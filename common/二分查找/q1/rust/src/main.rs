extern crate ans_lib;
use ans_lib::ans;

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
    let array = ans::gen_random_unique_array();
    let index = ans::gen_index(&array);
    let value = array[index];
    match find(&array, value) {
        Ok(ind) => {
            assert_eq!(ind, index);
            return Ok(());
        }
        Err(_) => {
            println!("Error: #{}, {}# {:?}", value, index, array);
            return Err(());
        }
    }
}

fn find(array: &Vec<i32>, value: i32) -> Result<usize, ()> {
    if array.len() == 0 {
        return Err(());
    }
    let mut left = 0;
    let mut right = array.len() - 1;

    // 一定要 <=
    while left <= right {
        let mid = (left + right) / 2;
        if array[mid] == value {
            return Ok(mid);
        } else if value < array[mid] {
            right = mid - 1;
        } else {
            left = mid + 1;
        }
    }
    return Err(());
}
