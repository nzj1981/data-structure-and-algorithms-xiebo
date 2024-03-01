//num2str_stk.rs 
//递归用栈实现
pub mod stack_mod;
use crate::stack_mod::Stack;

fn num2str_stk(mut num:i32, base: i32) -> String{
    let digits: [&str; 16] = ["0","1","2","3","4","5","6","7",
                             "8","9","A","B","C","D","E","F"];
    let mut rem_stack = Stack::new();
    while num > 0{
        if num < base{
            rem_stack.push(num);
        } else {
            rem_stack.push(num % base);
        }
        num /= base;
    }
    //出栈余数并组成字符串
    let mut num_str = "".to_string();
    while !rem_stack.is_empty(){
        num_str += digits[rem_stack.pop().unwrap() as usize];
    }
    num_str
}

fn main(){
    let num = 100;
    let sb = num2str_stk(num,2);
    let so = num2str_stk(num, 8);
    let sh = num2str_stk(num, 16);
    println!("{num} = b{sb}, o{so}, x{sh}");
}
