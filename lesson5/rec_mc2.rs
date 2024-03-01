//rec_mc2.rs 
//找零钱升级版解决耗时问题
use std::time::Instant;

fn rec_mc2(cashes: &[u32], amount: u32, min_cashes: &mut [u32]) -> u32{
    // 全用1元纸币的最小找零钱币数量
    let mut min_cashe_num = amount;

    if cashes.contains(&amount){
        //收集和当前待找零值相同的币种
        min_cashes[amount as usize] = 1;
        return 1;
    } else if min_cashes[amount as usize] > 0 {
        //找零值amount有最小找零纸币数，直接返回
        return min_cashes[amount as usize];
    } else {
         for c in cashes.iter().filter(|&&c| c <= amount).collect::<Vec<&u32>>(){
            let cashe_num = 1 + rec_mc2(cashes, amount - c, min_cashes);
            //更新最小找零纸币数
            if cashe_num < min_cashe_num {
                min_cashe_num = cashe_num;
                min_cashes[amount as usize] = min_cashe_num;
            }
        }
    }
    min_cashe_num
}

fn main(){
    let start = Instant::now();
    const AMOUNT:u32 = 90;
    let cashes: [u32;5] = [1, 5, 10, 20, 50];
    let mut min_cashes = Vec::from([0;AMOUNT as usize + 1]);//0元找零0张   
    let cashe_num = rec_mc2(&cashes, AMOUNT, &mut min_cashes);
    println!("need refund {cashe_num} cashes");

    println!("Time cost:\n\t{:?}ms, {:?}us {:?}ns",start.elapsed().as_millis(), start.elapsed().as_micros(), start.elapsed().as_nanos());
}
    
