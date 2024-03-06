//binary_search.rs 
//二分查找的实现方法
use std::time::Instant;
fn binary_search(nums: &[i32], num: i32) -> Option<usize> {
    let mut low = 0;
    let mut pos:usize = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    //注意是<=不是<
    while low <= high && !found {
        //let mid: usize = (low + high) >> 1;
        //若low + high 可能溢出,可转换为减法
        let mid: usize = low + ((high - low) >> 1);
        if num == nums[mid] {
            found = true;
            pos = mid;
        }else if num < nums[mid]{
            high = mid - 1; //num<中间值，省去后半部数据
        } else {
            low = mid + 1; //num>=中间值，省去前半部数据
        }
    }
    if found {Some(pos)} else {None}
}
//二分查找递归法实现
fn binary_search2(nums: &[i32], num: i32) -> bool {
    //基本情况1：项不存在
    if 0 == nums.len(){ return false; }
    let mid: usize = nums.len() >> 1;
    if num == nums[mid] {
        //基本情况2:项存在
        return true;
    } else if num < nums[mid] {
        return binary_search2(&nums[..mid],num);
    } else {
        return binary_search2(&nums[mid+1..],num);
    }
}

fn main(){
    let start = Instant::now();
    let nums = [1, 3, 8, 10, 15, 32, 44, 48, 50, 55, 60, 62, 64];//必须是有序的数组
    let mut target = 3;
    let found = binary_search(&nums, target);
    match found {
        Some(s) => println!("{target}'s index:{s}"),
        None => println!("{target} is not found"),
    }

    target = 44;
    let found = binary_search(&nums, target);
    match found {
        Some(s) => println!("{target}'s index:{s}"),
        None => println!("{target}'s index not found"),
    }
    target = 55;
    let found = binary_search(&nums, target);
    match found {
        Some(s) => println!("{target}'s index:{s}"),
        None => println!("{target}'s index not found"),
    }
    target = 95;
    let found = binary_search(&nums, target);
    match found {
        Some(s) => println!("{target}'s index:{s}"),
        None => println!("{target}'s index not found"),
    }
    target = 48;
    let found = binary_search2(&nums, target);
    println!("nums contains {target}: {found}");
    target = 8;
    let found = binary_search2(&nums, target);
    println!("nums contains {target}: {found}");
    target = 93;
    let found = binary_search2(&nums, target);
    println!("nums contains {target}: {found}");
    println!("Time cost:{:?}ms", start.elapsed().as_millis());
}
