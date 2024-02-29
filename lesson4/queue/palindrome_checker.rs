//回文检查
//palindrome_checker.rs 
pub mod deque_mod;
use crate::deque_mod::Deque;

fn palindrome_checker(pal: &str) -> bool {
    //数据队列
    let mut d = Deque::new(pal.len());
    for c in pal.chars(){
        let _r = d.add_rear(c);
    }

    let mut is_pal = true;
    while d.size() > 1 && is_pal {
        //出队首尾字符
        let head = d.remove_front();
        let tail = d.remove_rear();
        //比较首尾字符，若不同则非回文
        if head != tail {
            is_pal = false;
        }
    }
    is_pal
}

fn main(){
    let pal = "rustsur";
    let is_pal = palindrome_checker(pal);
    println!("\"{pal}\" is palindrome string: {is_pal}");

    let pal = "panda";
    let is_pal = palindrome_checker(pal);
    println!("\"{pal}\" is palindrome string: {is_pal}");
}
