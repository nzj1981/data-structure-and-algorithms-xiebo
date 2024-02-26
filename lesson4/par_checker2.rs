//par_checker2.rs
//同时检测多种开始符号和结束符号是否匹配
pub mod stack;
use crate::stack::Stack;

fn par_match(open: char, close: char) -> bool{
    let opens = "([{";
    let closers = ")]}";
    //在opens的开始符号要和closers的结束符号顺序对应
    opens.find(open) == closers.find(close)
}
 fn par_checker2(par: &str) -> bool{
     let mut char_list = Vec::new();
     for c in par.chars(){
         char_list.push(c);
     }
     let mut index = 0;
     let mut balance = true;
     let mut stack = Stack::new();
     while index < char_list.len() && balance {
         let c = char_list[index];
         //同时判断三种开始符号
         if '(' == c || '[' == c||'{' == c {
             stack.push(c);
         } else {
             if stack.is_empty(){
                 balance = false;
             } else {
                 // 比较当前括号和栈顶是否匹配
                 let top = stack.pop().unwrap();
                 if !par_match(top, c){
                     balance = false;
                 }
             }
         }
         index += 1;
     }
     balance && stack.is_empty()
 }
fn main(){
    let sa = "(){}[]";
    let sb = "(){)[}";
    let res1 = par_checker2(sa);
    let res2 = par_checker2(sb);
    println!("{sa} balanced:{res1}, {sb} balanced:{res2}");
}
