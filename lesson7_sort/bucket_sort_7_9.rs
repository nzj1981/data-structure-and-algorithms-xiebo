//bucket_sort_7_9.rs 
//桶排序-非比较排序算法
use std::time::Instant;
use std::fmt::Debug;
//桶定义
//hasher是一个函数，计算时传入
//values是数据容器，保存数据
struct Bucket<H, T> {
    hasher: H,
    values: Vec<T>,
}
impl<H, T> Bucket<H, T> {
    fn new(hasher: H, val: T) -> Bucket<H, T> {
        Bucket {
            hasher,
            values: vec![val]
        }
    }
}
//桶排序
fn bucket_sort<H, T, F>(nums: &mut [T], hasher: F)
    where H: Ord,
          T: Ord + Clone + Debug,
          F: Fn(&T) -> H {
    //准备多个桶
    let mut bks: Vec<Bucket<H, T>> = Vec::new();
    for val in nums.iter(){
        let hasher = hasher(&val);
        //数据入桶，对桶中数据二分搜索并排序
        match bks.binary_search_by(|bk| bk.hasher.cmp(&hasher)){
            Ok(idx) => bks[idx].values.push(val.clone()),
            Err(idx) => bks.insert(idx, Bucket::new(hasher, val.clone())),
        }
    }
    //拆桶，将所有数据整合到一个vec 
    let ret = bks.into_iter().flat_map(|mut bk|{
        bk.values.sort();
        bk.values
    }).collect::<Vec<T>>();
    //将排序数据拷回nums
    nums.clone_from_slice(&ret);
}
    

fn main(){
    let start = Instant::now();
    let mut nums = [0, 54, 32, 99, 18, 75, 31, 43, 4, 56, 21, 1, 100];
    bucket_sort(&mut nums, |t| t / 5);
    println!("Bucket sorted nums: {:?}", nums);
    println!("Time cost: {:?}ms", start.elapsed().as_millis());
}
