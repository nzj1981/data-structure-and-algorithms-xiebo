//binary_insertion_sort.rs 
//插入排序-采用二分查找加快插入排序
use std::time::Instant;
//二分插入排序
fn binary_insertion_sort(nums: &mut [i32]){
    let mut temp;
    let mut left;
    let mut mid;
    let mut right;

    for i in 1..nums.len(){
        //已排序数组左右边界
        left = 0;
        right = i - 1;

        //待排序数据
        temp= nums[i];
        //二分法找到temp的位置
        while left <= right {
            mid = (left + right) >> 1;
            if temp < nums[mid]{
                //防止出现right= 0 -1；
                if 0 == mid {break;}
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        //将数据后移，留出空位
        for j in (left..=i -1).rev(){
            nums.swap(j, j+1);
        }
        // 将temp插入空位
        if left != i {
            nums[left] = temp;
        }

    }
}

fn main(){
    let start = Instant::now();
    let mut nums = [ 1, 3, 2, 8, 6, 4, 9, 7, 5, 10];
    binary_insertion_sort(&mut nums);
    println!("binary insertion sort nums: {:?}", nums);
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
