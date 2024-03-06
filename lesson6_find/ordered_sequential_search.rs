//ordered_sequential_search.rs 
//在有序集合中顺序查找目标项
use std::time::Instant;

fn ordered_sequential_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos = 0;
    let mut found = false;
    let mut stop = false; //控制遇到有序数据时退出
    
    while pos < nums.len() && !found && !stop {
        if num == nums[pos] {
            found = true;
        } else if num < nums[pos] {
            stop = true; //数据有序，退出
        } else {
            pos += 1;
        }
    }
    if found {Some(pos)} else {
        None
    }
}

fn main(){
    let start = Instant::now();
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];
    let num = 14;
    let pos = ordered_sequential_search(&nums, num);
    match pos {
        Some(s) => println!("{num}'s index:{s}"),
        None => println!("None"),
    }
    println!("Time cost:{:?}", start.elapsed().as_millis());
}
