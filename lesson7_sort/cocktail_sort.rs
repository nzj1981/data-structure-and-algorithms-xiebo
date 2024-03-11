//cocktail_sort.rs 
//冒泡排序-鸡尾酒排序法，从左向右冒泡和从右向左冒泡
use std::time::Instant;

fn cocktail_sort(nums: &mut [i32]){
    if nums.len()<2 {return;}
    //bubble控制是否继续冒泡
    let mut bubble = true;
    let len = nums.len();
    for i in 0..(len >>1){
        if bubble {
            bubble = false;
            //从左到右冒泡
            for j in i..(len -i -1){
                if nums[j] > nums[j + 1]{
                    nums.swap(j, j+1);
                    bubble = true
                }
            }
            //从右到左冒泡
            for j in (i+1..=(len -i -1)).rev(){
                if nums[j] < nums[j-1]{
                    nums.swap(j, j-1);
                    bubble = true
                }
            }
        } else {
            break;
        }
    }
}


fn main(){
    let start = Instant::now();
    let mut nums = [1, 3, 2, 8, 3, 6, 4, 9, 5, 10, 6, 7];
    cocktail_sort(&mut nums);
    println!("Cocktail sorted nums {:?}", nums);
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
