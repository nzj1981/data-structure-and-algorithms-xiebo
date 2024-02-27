//检测括号是否匹配
pub fn par_checker3(infix: &str) -> bool{
    let mut char_list = Vec::new();
    for c in infix.chars(){char_list.push(c);}

    let mut index = 0;
    let mut balance = true;
    let mut stack = Vec::new();

    while index < char_list.len() && balance {
        let c = char_list[index];
        if '(' == c || '[' == c || '{' == c {
            stack.push(c);
        }

        if ')' == c || ']' == c || '}' == c {
            if stack.is_empty(){
                balance = false;
            } else {
                let top = stack.pop().unwrap();
                if !par_match(top, c) {
                    balance = false;
                }
            }
        }
        index += 1;
    } 
    balance && stack.is_empty()
}
//同时检测多种括号
fn par_match(open: char, close: char) -> bool {
    let opens = "([{";
    let closers = ")]}";
    opens.find(open) == closers.find(close)
}

