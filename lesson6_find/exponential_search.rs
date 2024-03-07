//exponential_search.rs 
//指数查找适合已排序且无边界的数据
use std::time::Instant;
fn binary_search(nums: &[i32], num: i32) -> bool {
    let mut low =0;
    let mut high = nums.len() - 1;
    let mut found = false;
    while low <= high && !found {
        let mid: usize = (low + high) >> 1;
        if num == nums[mid] {
            found = true;
        } else if num < nums[mid] {
            high = mid - 1;
        } else {
            low = mid + 1;
        }
    }
    found
}

fn exponential_search(nums: &[i32], target: i32) -> bool {
    if nums.is_empty(){return false;}
    let size = nums.len();
    //逐步找到上界
    let mut high = 1usize;
    while high < size && nums[high] < target {
        high <<=1;
    }
    //上界的一半一定可以作为下界
    let low = high >> 1;
    //区间内二分搜索加速查找
    binary_search(&nums[low..size.min(high+1)], target)
}

fn main(){
    let start = Instant::now();
    let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];
    let target = 27;
    let found = exponential_search(&nums, target);
    println!("nums contains {target}: {found}");
    let target = 35;
    let found = exponential_search(&nums, target);
    println!("nums contains {target}: {found}");
    let target = 2;
    let found = exponential_search(&nums, target);
    println!("nums contains {target}: {found}");
    println!("Time cost: {}ms", start.elapsed().as_millis());
}
