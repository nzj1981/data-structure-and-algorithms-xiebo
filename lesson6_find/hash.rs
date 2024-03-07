//hash.rs 
//ASCII哈希函数的实现
//https://www.cnblogs.com/chenguifeng/p/12243999.html折叠函数学习
use std::time::Instant;
fn hash1(ascii_str: &str, size: usize) -> usize {
    let mut sum = 0;
    for c in ascii_str.chars(){
        sum += c as usize;
    }
    sum % size
}
//hash1的迭代形式
fn hash1_iter(ascii_str: &str, size: usize) -> usize {
    ascii_str.chars()
              .fold(0, |acc, c| acc + c as usize)
              % size
}
fn hash2(ascii_str: &str, size: usize) -> usize{
    let mut sum = 0;
    for (i, c) in ascii_str.chars().enumerate(){
        sum += (i + 1) * (c as usize);
    }
    sum % size
}
//hash2迭代形式
fn hash2_iter(ascii_str: &str, size: usize) -> usize{
    ascii_str.chars()
             .enumerate()
             .fold(0, |acc, (i, c)| acc + (i + 1) * (c as usize))
             % size
}
fn main(){
    let start = Instant::now();
    let s1 = "rust";
    let s2 = "Rust";
    let s3 = "java";
    let s4 = "Java";
    let size = 11; //槽数
    let h1 = hash1(s1, size);
    let h2 = hash1_iter(s2, size);
    println!("{s1} in slot {h1}, {s2} in slot {h2}");
    let h3 = hash2(s3, size);
    let h4 = hash2_iter(s4, size);
    println!("{s3} in slot {h3}, {s4} in slot {h4}");
    println!("Time cost: {}ms", start.elapsed().as_millis());
}
