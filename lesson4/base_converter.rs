//十转二、八、十六进制转换
//base_converter.rs
fn base_converter(mut dec_num: u32, base: u32) -> String{
    //digits对应各种余数的字符形式，尤其是10~15
    let digits = ['0', '1', '2','3','4', '5', '6', '7', '8', '9', 'A', 'B', 'C', 'D', 'E', 'F'];
    let mut rem_stack = Vec::new();

    //余数入栈
    while dec_num > 0 {
        let rem = dec_num % base;
        rem_stack.push(rem);
        dec_num /= base;
    }

    //余数出栈并取对应字符以拼接成字符串
    let mut base_str = "".to_string();
    while !rem_stack.is_empty(){
        let rem = rem_stack.pop().unwrap() as usize;
        base_str += &digits[rem].to_string();
    }
    base_str
}

fn main(){
    let num1 = 10;
    let num2 = 43;
    let bin_str: String = base_converter(num1, 2);
    let hex_str: String = base_converter(num2, 16);

    println!("{num1} = b{bin_str}, {num2} = x{hex_str}");
    // 10=b1010,43=x2B
}
