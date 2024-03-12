//insertion_sort.rs 
//插入排序
use std::time::Instant;
fn insertion_sort(nums: &mut [i32]){
    if nums.len() < 2 {return;}

    for i in 1..nums.len(){
        let mut pos = i;
        let curr = nums[pos];
        while pos > 0 && curr < nums[pos - 1]{
            nums[pos] = nums[pos - 1]; //向后移动数据
            pos -= 1;
        }
        nums[pos] = curr; //插入数据
    }
}
fn main(){
    let start = Instant::now();
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21];
    insertion_sort(&mut nums);
    println!("Insertion sorted nums: {:?}", nums);

    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
