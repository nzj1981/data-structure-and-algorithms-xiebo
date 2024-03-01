//rec_mc1.rs 
//找零钱过程就是递归
//cashes:代表要找零钱张数,amount：代表要找的零钱
use std::time::Instant;
fn rec_mc1(cashes: &[u32], amount: u32) -> u32 {
    //全用1 元纸币时的最少找零纸币数
    let mut min_cashes = amount;
    if cashes.contains(&amount){
        return 1;
    } else {
        //提取符合条件的币值（找零币值肯定小于等于找零值）
        for c in cashes.iter().filter(|&&c| c <= amount).collect::<Vec<&u32>>(){
            // amount 减去c，表示使用了一个面额为c的纸币，所以要加1 
            let num_cashes = 1 + rec_mc1(&cashes, amount - c);
            //num_cashes若比min_cashes小则更新
            if num_cashes < min_cashes {
                min_cashes = num_cashes;
            }
        }
    }
    min_cashes
}

fn main(){
    let start = Instant::now();
    //cashes保存各种面额的纸币
    let cashes = [1, 5, 10, 20, 50];
    let amount = 31u32; //68用时在86秒
    let cashes_num = rec_mc1(&cashes, amount);
    println!("need refund {cashes_num} cashes");
    println!("Time cost:\n\t{:?}ms, {:?}us {:?}ns",start.elapsed().as_millis(), start.elapsed().as_micros(), start.elapsed().as_nanos());
}
