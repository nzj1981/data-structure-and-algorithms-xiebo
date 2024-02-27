//divide_by_two.rs 
//简单的十进制转二进制

fn divide_by_two(mut dec_num: u32) -> String{
    //用栈保存余数rem 
    let mut rem_stack = Vec::new();

    //余数rem入栈
    while dec_num > 0 {
        let rem = dec_num % 2;
        rem_stack.push(rem);
        dec_num /=2;
    }

    //栈中元素出栈，组成字符串
    let mut bin_str = "".to_string();
    while !rem_stack.is_empty(){
        let rem = rem_stack.pop().unwrap().to_string();
        bin_str += &rem;
    }
    bin_str
}

fn main(){
    let num = 10;
    let bin_str:String = divide_by_two(num);
    println!("{num} = b{bin_str}");
    //10=b1010
}
