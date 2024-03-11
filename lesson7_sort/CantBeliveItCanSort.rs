//CantBeliveCanSort.rs 
//冒泡排序-类似于插入排序,不需要处理边界下标值的算法
use std::time::Instant;

//升序排序
fn cbic_sort1(nums: &mut [i32]){
    for i in 0..nums.len(){
        for j in 0..nums.len(){
            if nums[i] < nums[j]{
                nums.swap(i, j);
            }
        }
    }
}
//升序排序优化版
fn cbic_sort2(nums: &mut [i32]){
    if nums.len() < 2{return;}
    for i in 1..nums.len(){
        for j in 0..i{
            if nums[i] < nums[j]{
                nums.swap(i, j);
            }
        }
    }
}
//降序排序
fn cbic_sort3(nums: &mut [i32]){
    for i in 0..nums.len(){
        for j in 0..nums.len(){
            if nums[i] > nums[j]{
                nums.swap(i, j);
            }
        }
    }
}
//降序排序优化版
fn cbic_sort4(nums: &mut [i32]){
    if nums.len() < 2{return;}
    for i in 1..nums.len(){
        for j in 0..i{
            if nums[i]>nums[j]{
                nums.swap(i,j);
            }
        }
    }
}

fn main(){
    let start = Instant::now();
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    cbic_sort1(&mut nums);
    println!("cbic sorted nums: {:?}", nums);
    let mut nums: Vec<_> = (1..11).filter(|x| x % 2 ==0).collect::<Vec<_>>();
    cbic_sort2(&mut nums);
    println!("Cbic sorted nums: {:?}", nums);
    let mut nums = [54, 32, 99, 18, 75, 31, 43, 56, 21, 22];
    cbic_sort3(&mut nums);
    println!("Cbic sorted nums:{:?}", nums);
    let mut nums = (1..10).collect::<Vec<_>>();
    cbic_sort4(&mut nums);
    println!("Cbic sorted nums: {:?}", nums);
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
