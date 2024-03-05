//动态规划-找零钱数并且显示零钱面额
//dp_rec_mc_show.rs 
use std::time::Instant;
//使用cashes_used收集使用过的各种面额纸币
fn dp_rec_mc_show(cashes: &[u32],
                  amount: u32,
                  min_cashes: &mut [u32],
                  cashes_used: &mut [u32]) -> u32 {
    for denm in 1..=amount {
        let mut min_cashes_num = denm;
        let mut used_cashe = 1; //最小面额是1元
        for c in cashes.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>(){
            let index = (denm - c) as usize;
            let cashe_num = 1 + min_cashes[index];
            if cashe_num < min_cashes_num{
                min_cashes_num = cashe_num;
                used_cashe = *c;
            }
        }
        //更新各种面额对应的最小纸币数
        min_cashes[denm as usize] = min_cashes_num;
        cashes_used[denm as usize] = used_cashe;
    }
    min_cashes[amount as usize]
}

//打印输出各面额纸币
fn print_cashes(cashes_used: &[u32], mut amount: u32) {
    while amount > 0 {
        let curr = cashes_used[amount as usize];
        println!("￥{curr}");
        amount -= curr;
    }
}


fn main(){
    let start = Instant::now();
    let cashes = [1, 2, 5, 10, 20, 50];
    const AMOUNT: u32 = 90;
    let mut min_cashes = [0;AMOUNT as usize + 1];
    let mut cashes_used = [0;AMOUNT as usize + 1];
    let min_cashes_num = dp_rec_mc_show(&cashes, AMOUNT, &mut min_cashes, &mut cashes_used);
    println!("Refund for ￥{AMOUNT} requires {min_cashes_num} cashes:");
    print_cashes(&cashes_used, AMOUNT);

    println!("Time cost:\n\t{:?}ms",start.elapsed().as_millis());
}
