//实现非括号字符自动忽略
//par_checker3.rs
fn par_checker3(par: &str) -> bool{
    let mut char_list = Vec::new();
    for c in par.chars(){char_list.push(c);}

    let mut index = 0;
    let mut balance = true;
    let mut stack = Vec::new();

    while index < char_list.len() && balance{
        let c = char_list[index];
        //将开始符号入栈
        if '(' == c || '[' == c || '{' == c {stack.push(c);}
        //如果是结束符号，则判断是否平衡
        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty(){
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c){balance = false;}
            }
        }
        //非括号字符直接跳过
        index += 1;
    }
    balance && stack.is_empty()
}

// 同时检测多种开始符号和结束符号是否匹配
fn par_match(open: char, close: char) -> bool{
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

fn main(){
    let sa = "(2+3){func}[abc]";let sb = "(2+3)*(3-1";
    let res1 = par_checker3(sa);
    let res2 = par_checker3(sb);
    println!("{sa} balance: {res1},{sb} balanced:{res2}");
}
