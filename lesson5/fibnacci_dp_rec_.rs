//动态规划解决斐波那契数列问题
//fibnacci_dp_rec.rs 
use std::time::Instant;

fn fibonacci_dp(n: u32) -> u32{
    if 0 == n { return 0;}
    if 1 == n || 2 == n {
        return n;
    } else {
    //只用两个位置来保存值，节约内存
    let mut dp = [1, 1];
    for i in 2..=n {
        let idx1 = (i % 2) as usize;
        let idx2 = ((i - 1) % 2) as usize;
        let idx3 = ((i - 2) % 2) as usize;
        dp[idx1] = dp[idx2] + dp[idx3];
    }
    dp[((n - 1) % 2) as usize]
    }
}
fn fibonacci_rec(n: u32) -> u32 {
    if 0 == n {return 0;}
    if 1 == n || 2 == n{
        return 1;
    } else {
        fibonacci_rec(n - 1) + fibonacci_rec( n - 2)
    }
}
fn main(){
    let start = Instant::now();
    let fib:u32 = 99;
   // println!("fib({fib}): {}", fibonacci_dp(fib));
    println!("fib({fib}): {}", fibonacci_rec(fib));
    println!("Time cost:{:?}ms", start.elapsed().as_millis());
}
