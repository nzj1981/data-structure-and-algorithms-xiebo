//shell_sort.rs 
//希尔排序又称递减增量排序，是一种非稳定排序算法，原理是将原始集合划分为
//多个较小的子集合，然后对每个子集合进行插入排序
use std::time::Instant;

fn shell_sort(nums: &mut [i32]){
    //插入排序函数（内部），数据相隔距离为gap
    fn ist_sort(nums: &mut [i32], start: usize, gap: usize){
        let mut i = start + gap;

        while i < nums.len(){
            let mut pos = i;
            let curr = nums[pos];
            while pos >= gap && curr < nums[pos - gap]{
                nums[pos] = nums[pos - gap];
                pos -= gap;
            }
            nums[pos] = curr;
            i += gap;
        }
    }
    if nums.len() < 2 {return;}
    //每次让gap减少一半直到1
    let mut gap = nums.len() >> 1;
    while gap > 0 {
        for start in 0..gap {
            ist_sort(nums, start, gap);
        }
        gap /= 2;
    }
}

fn main(){
    let start = Instant::now();
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    shell_sort(&mut nums);
    println!("Shell sorted nums: {:?}", nums);

    println!("Time cost: {:?}", start.elapsed().as_millis());
}
