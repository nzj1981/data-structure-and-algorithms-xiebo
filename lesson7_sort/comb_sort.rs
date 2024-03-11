//comb_sort.rs 
//冒泡排序-梳排序方法,属于不稳定排序法
use std::time::Instant;

fn comb_sort(nums: &mut [i32]) {
    if nums.len() < 2 {return;}
    let mut i;
    let mut gap: usize = nums.len();
    //大致排序,使数据基本有序
    while gap>0{
        gap = (gap as f32 * 0.8) as usize;
        i = gap;
        while i < nums.len(){
            if nums[i - gap] > nums[i]{
                nums.swap(i-gap, i);
            }
            i +=1;
        }
    }
    //细致调节部分无序数据,exchange控制是否继续交换数据
    let mut exchange = true;
    while exchange {
        exchange = false;
        i = 0;
        while i < nums.len() -1 {
            if nums[i] > nums[i + 1]{
                nums.swap(i, i+1);
            }
            i += 1;
        }
    }

}

fn main(){
    let start = Instant::now();
    let mut nums = [1, 2, 8, 3, 4, 9, 5, 6, 7];
    comb_sort(&mut nums);
    println!("Comb sorted nums {:?}", nums);
    println!("Time cost: {:?}", start.elapsed().as_millis());
}
