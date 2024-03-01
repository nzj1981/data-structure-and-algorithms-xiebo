//nums_sum12.rs 
//递归三大定律
/*
*1.递归算法必须具有基本情况
*2.递归算法必须向基本情况靠近
*3.递归算法必须以递归方式调用自身
*/

fn nums_sum1(nums: &[i32]) -> i32{
    if 1 == nums.len(){
        nums[0]
    } else {
        let first = nums[0];
        first + nums_sum1(&nums[1..])
    }
}
fn nums_sum2(nums: &[i32]) -> i32 {
    if 1 == nums.len(){
        nums[0]
    } else {
        let last = nums[nums.len() - 1];
        last + nums_sum2(&nums[..nums.len() - 1])
    }
}

fn main(){
    let nums = [2, 1, 7, 4, 5];
    let sum1 = nums_sum1(&nums);
    let sum2 = nums_sum2(&nums);
    println!("sum1 = {sum1}, sum2 = {sum2} ");


    let nums = [-1, 7, 1, 2, 5, 4, 10, 100];
    let sum1 = nums_sum1(&nums);
    let sum2 = nums_sum2(&nums);
    println!("sum1 = {sum1}, sum2 = {sum2} ");
}
