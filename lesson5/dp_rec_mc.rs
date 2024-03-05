//动态规划-找零钱纸币数
//dp_rec_mc.rs 
use std::time::Instant;
fn dp_rec_mc(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32 {
    //动态收集从1到amount的最小找零纸币数
    for denm in 1..=amount {
        //此min_cashe_num等于全用1元纸币找零的纸币数
        let mut min_cashe_num = denm;
        for c in cashes.iter().filter(|&&c| c <= denm).collect::<Vec<&u32>>(){
            let index = (denm -c) as usize;
            //加1是因为当前最小找零数等于上一最小找零数加1张c面额纸币
            let cashe_num = min_cashes[index] + 1;
            if cashe_num < min_cashe_num{
                min_cashe_num = cashe_num;
            }
        }
        min_cashes[denm as usize] = min_cashe_num;
    }
    //因为收集了所有的最小找零纸币数，所以直接返回
    min_cashes[amount as usize]
}

fn main(){
    let start = Instant::now();
    const AMOUNT:u32 = 90;
    let cashes = [1, 2, 5, 10, 20, 50];
    let mut min_cashes = vec![0;AMOUNT as usize + 1];
    let cashe_num = dp_rec_mc(&cashes, AMOUNT, &mut min_cashes);
    println!("Refund for ￥{AMOUNT} need {cashe_num} cashes");

    println!("Time cost:\n\t{:?}ms",start.elapsed().as_millis());
    
}
