//counting_sort_7_10.rs 
//计数排序法
use std::time::Instant;
fn counting_sort(nums: &mut [usize]){
    if nums.len() < 2 { return;}
    //桶数量为nums中最大值加1,保证数据都有桶放
    let max_bkt_num = 1 + nums.iter().max().unwrap();
    let mut counter = vec![0; max_bkt_num];

    //将数据标记到桶
    for &v in nums.iter(){
        counter[v] += 1;
    }

    // 数据写回原nums切片
    // j表示nums的下标
    let mut j = 0;
    for i in 0..max_bkt_num {
        while counter[i] > 0 {
            nums[j] = i;
            counter[i] -= 1;
            j += 1;
        }
    }
}


fn main(){
    let start = Instant::now();
    let mut nums = [54, 32, 99, 18, 75, 31, 0, 43, 56, 18, 21, 22,32];
    counting_sort(&mut nums);
    println!("Counting sorted nums: {:?}", nums);
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
