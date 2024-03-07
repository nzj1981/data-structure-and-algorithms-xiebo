//interpolation_search.rs 
//内插法查找 (y-y0)/(x-x0)=(y1-y0)/(x1-x0) => x = (y-y0)*(x1-x0)/(y1-y0) +x0 
use std::time::Instant;
//查找目标值所在位置且源是有序增长数组
fn interpolation_search(nums: &[i32], target: i32) -> Option<usize> {
    let mut _found = false;
    let mut inter_idx = 0usize;
    if nums.is_empty(){
        return None;
    }
    //查询范围
    let mut low = 0usize;
    let mut high = nums.len() - 1;
    loop {
        let low_val = nums[low];
        let high_val = nums[high];
        //high和low相交或者目标值小于小边界或大于大边界,则不存在
        if high <= low || target < low_val || target > high_val {
            _found = false;
            break;
        }
        //计算插值位置
        let offset = (target - low_val) * ( high - low) as i32 / (high_val - low_val);
        let interpolant = low + offset as usize;
        // 更新上下界high low
        if nums[interpolant] > target {
            high = interpolant - 1;
        } else if nums[interpolant] < target {
            low = interpolant + 1;
        } else {
            //找到值
            inter_idx = interpolant;
            _found = true;
            break;
        }
        
    }
    if _found {Some(inter_idx)} else {None}
}


fn main(){
    let start = Instant::now();
    let nums = [1, 9, 10, 15, 16, 17, 19, 23, 27, 28, 29, 30, 32, 35];//必须有序增长数组
    //let nums= Vec::new();
    let target = 27;
    let found = interpolation_search(&nums, target);
    match found {
        Some(v) => println!("{target}'s index:{v}"),
        None => println!("{target}'s index not found!"),
    }
    let target = 36;
    let found = interpolation_search(&nums, target);
    match found {
        Some(v) => println!("{target}'s index:{v}"),
        None => println!("{target}'s index not found!"),
    }
    let target = 13;
    let found = interpolation_search(&nums, target);
    match found {
        Some(v) => println!("{target}'s index:{v}"),
        None => println!("{target}'s index not found!"),
    }
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
