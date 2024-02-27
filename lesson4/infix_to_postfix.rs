//中缀转后缀
// infix_to_postfix.rs
pub mod parentheses;
pub mod stack;
use crate::stack::Stack;
use crate::parentheses::par_checker3;
use std::collections::HashMap;

fn infix_to_postfix(infix: &str) -> Option<String> {
    //括号匹配检验
    if !par_checker3(infix) {return None;}
    //设置各个符号的优先级
    let mut prec = HashMap::new();
    prec.insert("(",1);prec.insert(")", 1);
    prec.insert("+",2);prec.insert("-", 2);
    prec.insert("*",3);prec.insert("/", 3);
    
    //ops保存操作符号，postfix保存后缀表达式
    let mut ops = Stack::new();
    let mut postfix = Vec::new();
    //通过空格把中缀表达式转换成切片
    for token in infix.split_whitespace(){
        if ("A" <= token && token <= "Z") || ("0" <= token && token <= "9"){
            //0-9,A-Z 范围字符入栈
            postfix.push(token);
        } else if "(" == token {
            //遇到开括号，将操作符入栈
            ops.push(token);
        } else if ")" == token {
            //遇到闭括号，将操作符入栈
            let mut top = ops.pop().unwrap();
            while top != "(" {
                postfix.push(top);
                top = ops.pop().unwrap();
            }
        } else {
            //比较符号的优先级决定操作符号是否加入postfix
            while !ops.is_empty() && prec[ops.peek().unwrap()] >= prec[token]{
                postfix.push(ops.pop().unwrap());
            }
            ops.push(token);
        }
    }

    //剩下的操作数入栈
    while !ops.is_empty(){
        postfix.push(ops.pop().unwrap());
    }
    //出栈并组成字符串
    let mut postfix_str = "".to_string();
    for c in postfix {
        postfix_str += &c.to_string();
        postfix_str += " ";
    }
    Some(postfix_str)
}

fn main(){
    let infix = "( A + B ) * ( C + D )"; //中缀表达式每个字符间一定要有空格
    let postfix = infix_to_postfix(infix);
    match postfix {
        Some(v) => {
            println!("{infix} -> {v}");
        },
        None => {
            println!("{infix} isn't a corrent infix string");
        },
    }
}
