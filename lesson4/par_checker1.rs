//检查括号匹配成对出现
pub mod stack;
use crate::stack::Stack;

fn par_checker1(par: &str)-> bool{
    //将字符添加到vec中
    let mut char_list = Vec::new();
    for c in par.chars(){ char_list.push(c);}

    let mut index = 0;
    let mut balance = true; //括号是否匹配（平衡）标识
    let mut stack = Stack::new();
    
    while index < char_list.len() && balance {
        let c = char_list[index];

        if '(' == c {
            //如果为开始符号,入栈
            stack.push(c);
        } else {
            //如果为结束符号，判断栈是否为空
            if stack.is_empty(){
                //栈为空，所以括号不匹配
                balance = false;
            } else {
                let _r = stack.pop();
            }
        }
        index += 1;
    }
    //仅当平衡且栈为空时，括号才是匹配的
    balance && stack.is_empty()
}

fn main(){
    let sa = "()(())";
    let sb = "(())((())";
    let res1 = par_checker1(sa);
    let res2 = par_checker1(sb);
    println!("{sa} balance:{res1}, {sb} balance:{res2}");
    //()(()) balance:true, (())((()) balance:false
    
}
