//selection_sort_7_7.rs 
//选择排序是对冒泡排序的改进，每次遍历集合时只进行一次数据交换。
use std::time::Instant;
fn selection_sort(nums: &mut Vec<i32>){
    //待排序数据个数
    let mut left = nums.len() - 1;
    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max]{
                //当前轮次最大值下标
                pos_max = i;
            }
        }
        // 数据交换，完成一个数据的排序，待排序数据量减1
        nums.swap(left, pos_max);
        left -= 1;
    }
}

fn main(){
    let start = Instant::now();
    let mut nums = vec![54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    selection_sort(&mut nums);
    println!("Selection sorted nums: {:?}", nums);
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
