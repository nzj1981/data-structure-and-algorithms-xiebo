//tim_sort_without_gallop_7_12.rs 
//蒂姆排序-非加速版
/*
 * 情况1: A > B，所以不合并                      情况2: A < B，所以 B 和 A 合并
 *       |                 |                            |                 |
 *  B -> |   [xxxxxxxxx]   |                       B -> | [xxxxxxxxxxxxx] |
 *       |                 |                            |                 |
 *  A -> | [xxxxxxxxxxxxx] |                       A -> |   [xxxxxxxxx]   |
 *       |                 |                            |                 |
 *
 * 情况3: A > B + C、 B > C，所以不合并          情况4: A > B + C、 B < C，所以 B 和 C 合并
 *       |                 |                            |                 |
 *  C -> |     [xxx]       |                       C -> |   [xxxxxxx]     |
 *       |                 |                            |                 |
 *  B -> |   [xxxxxxxx]    |                       B -> |     [xxx]       |
 *       |                 |                            |                 |
 *  A -> | [xxxxxxxxxxxxx] |                       A -> | [xxxxxxxxxxxxx] |
 *       |                 |                            |                 |
 *
 *
 * 情况5: A < B + C、 A > C，所以 B 和 C 合并    情况6: A < B + C、 C > A，所以 B 和 A 合并
 *       |                 |                            |                 |
 *  C -> |    [xxxxxx]     |                       C -> |  [xxxxxxxxxxx]  |
 *       |                 |                            |                 |
 *  B -> |   [xxxxxxxx]    |                       B -> | [xxxxxxxxxxxxx] |
 *       |                 |                            |                 |
 *  A -> | [xxxxxxxxxxxxx] |                       A -> |   [xxxxxxxxx]   |
 *       |                 |                            |                 |
 */
use std::time::Instant;

//参与序列合并的最短长度，短于它则利用插入排序
const MIN_MERGE: usize = 64;

//排序状态体
struct SortState<'a> {
    list: &'a mut [i32],
    runs: Vec<Run>,
    pos: usize,
}

//定义Run实体,保存run在list中的起始下标和长度
#[derive(Debug, Copy, Clone)]
struct Run {
    pos: usize,
    len: usize,
}
// merge_lo 排序状态体
#[derive(Debug)]
struct MergeLo<'a> {
    list_len: usize, //待排序集合长度
    first_pos: usize,//run1的起始位置
    first_len: usize, //run1的长度
    second_pos: usize,//run2的起始位置
    dest_pos: usize, //排序结果的下标位置
    list: &'a mut [i32],//待排序集合的部分区间
    temp: Vec<i32>, //临时存储，长度设置run1、run2中较短值
                    //放最后，便于内存对齐优化。
}
//merge_hi 排序状态体
struct MergeHi<'a> {
    first_pos: isize,
    second_pos: isize,
    dest_pos: isize,
    list: &'a mut [i32],
    temp: Vec<i32>,
}

//计算minrun，实际范围为[32, 64]
fn calc_minrun(len: usize) -> usize {
    //如果len的低位有任何一位为1，r就会置为1 
    let mut r = 0;
    let mut new_len = len;
    while new_len >= MIN_MERGE{
        r |= new_len & 1;
        new_len >>= 1;
    }
    new_len + r 
}
//计算run的起始下标,并将逆序run转成正序
fn count_run(list: &mut [i32]) -> usize {
    let (ord, pos) = find_run(list);
    if ord {
        list.split_at_mut(pos).0.reverse();
    }
    pos 
}

//根据list[i]与list[i+1]的关系判断是
//升序还是降序，同时返回序列关系转折点下标
fn find_run(list: &[i32]) -> (bool, usize) {
    let len = list.len();
    if len < 2 {
        return ( false, len);
    }

    let mut pos = 1;
    if list[1] < list[0] {
        //降序list[i+1] < list[i]
        while pos < len - 1 && list[pos + 1] < list[pos] {
            pos += 1;
        }
        (true, pos + 1)
    } else {
        //升序 list[i+1] >= list[i]
        while pos < len - 1 && list[pos + 1] >= list[pos] {
            pos += 1;
        }
        (false, pos + 1)
    }
}

//二分插入排序
fn binary_insertion_sort(list: &mut [i32]) {
    let mut temp; //待排序数据
    let mut left; //左边界
    let mut mid; //中间位置
    let mut right; // 右边界

    for i in 1..list.len(){
        left = 0;
        right = i - 1;
        temp = list[i];

        //二分法找到temp的位置
        while left <= right {
            mid = left + ((right - left) >> 1);
            if temp < list[mid] {
                //防止出现 right = 0 - 1 
                if 0 == mid {break;}
                right = mid - 1;
            } else {
                left = mid + 1;
            }
        }
        //将数据后移，留出空位
        for j in (left..=i - 1).rev(){
            list.swap(j, j + 1);
        }
        //将temp放入空位
        if left != i {
            list[left] = temp;
        }
    }
}
// A，B，C 归并排序
fn merge_sort(list: &mut [i32], first_len: usize, second_len: usize) {
    if 0 == first_len || 0 == second_len { return;}

    if first_len > second_len {
        // B 和 C 合并，借助temp从list末尾开始合并
        merge_hi(list, first_len, second_len);
    } else {
        // B 和 A 合并，借助temp从list首部开始合并
        merge_lo(list, first_len);
    }
}
//合并A，B为一个run 
fn merge_lo(list: &mut [i32], first_len: usize) {
    unsafe {
        let mut state = MergeLo::new(list, first_len);
        state.merge();
    }
}
impl<'a> MergeLo<'a> {
    unsafe fn new(list: &'a mut [i32], first_len: usize) -> Self {
        let mut ret_val = MergeLo {
            list_len: list.len(),
            first_pos: 0,
            first_len: first_len,
            second_pos: first_len, //run1 和 run2 挨着，所以run2 起始位置就等于run1 长度
            dest_pos: 0, //排序结果写回原始集合，且从run1的起始位置开始写
            list: list,
            temp: Vec::with_capacity(first_len),
        };
        // 把run1 复制到temp 
        ret_val.temp.set_len(first_len);
        for i in 0..first_len {
            ret_val.temp[i] = ret_val.list[i];
        }
        ret_val
    }
    //归并排序
    fn merge(&mut self) {
        while self.second_pos > self.dest_pos && self.second_pos < self.list_len {
            debug_assert!(self.first_pos + (self.second_pos - self.first_len) == self.dest_pos);
            if self.temp[self.first_pos] > self.list[self.second_pos] {
                self.list[self.dest_pos] = self.list[self.second_pos];
                self.second_pos += 1;
            } else {
                self.list[self.dest_pos] = self.temp[self.first_pos];
                self.first_pos += 1;
            }
            self.dest_pos += 1;

        }
    }
}

impl<'a> Drop for MergeLo<'a> {
    fn drop(&mut self) {
        unsafe {
            // 将temp中剩余的值放到list高位
            if self.first_pos < self.first_len {
                for i in 0..(self.first_len - self.first_pos) {
                    self.list[self.dest_pos + i] = self.temp[self.first_pos + i];
                }
            }
            self.temp.set_len(0);
        }
    }
}

//合并 B，C 为一个run
fn merge_hi(list: &mut [i32], first_len: usize, second_len: usize) {
    unsafe {
        let mut state = MergeHi::new(list, first_len, second_len);
        state.merge();
    }
}
impl<'a> MergeHi<'a> {
    unsafe fn new(list: &'a mut [i32], first_len: usize, second_len: usize) -> Self {
        let mut ret_val = MergeHi {
            first_pos: first_len as isize - 1,
            second_pos: second_len as isize - 1,
            dest_pos: list.len() as isize - 1, //从末尾开始排序
            list: list,
            temp: Vec::with_capacity(second_len),
        };
        //把run2 复制到temp 
        ret_val.temp.set_len(second_len);
        for i in 0..second_len {
            ret_val.temp[i] = ret_val.list[i + first_len];
        }
        ret_val
    }
    //归并排序
    fn merge(&mut self) {
        while self.first_pos < self.dest_pos && self.first_pos >= 0 {
            debug_assert!(self.first_pos + self.second_pos + 1 == self.dest_pos);
            if self.temp[self.second_pos as usize] >= self.list[self.first_pos as usize] {
                self.list[self.dest_pos as usize] = self.temp[self.second_pos as usize];
                self.second_pos -= 1;
            } else {
                self.list[self.dest_pos as usize] = self.list[self.first_pos as usize];
                self.first_pos -= 1;
            }
            self.dest_pos -= 1;
        }
    }
}

impl<'a> Drop for MergeHi<'a> {
    fn drop(&mut self) {
        unsafe {
            //将 temp 中剩余的值放到list的低位
            if self.second_pos >= 0 {
                let size = self.second_pos + 1;
                let src = 0;
                let dest = self.dest_pos - size;
                for i in 0..size {
                    self.list[(dest + i) as usize] = self.temp[(src + i) as usize];
                }
            }
            self.temp.set_len(0);
        }
    }
}

impl<'a> SortState<'a> {
    fn new(list: &'a mut [i32]) -> Self {
        SortState {
            list: list,
            runs: Vec::new(),
            pos: 0,
        }
    }

    fn sort(&mut self) {
        let len = self.list.len();
        //计算minrun
        let minrun = calc_minrun(len);
        
        while self.pos < len {
            let pos = self.pos;
            let mut run_len = count_run(self.list.split_at_mut(pos).1);
            //判断剩下的元素数是否小于minrun，
            //如果是，则run_minlen = len - pos 
            let run_minlen = if minrun > len - pos {
                len - pos
            } else {
                minrun
            };
            //如果扪很短，则扩充它的长度到run_minlen
            //同时扩充后的run需要有序，所以使用二分插入排序
            if run_len < run_minlen {
                run_len = run_minlen;
                let left = self.list
                               .split_at_mut(pos).1 
                               .split_at_mut(run_len).0;
                binary_insertion_sort(left);
            }
            
            //将run 入栈、各run的长度不同
            self.runs.push(Run {
                pos: pos,
                len: run_len,
            });
            //找到下一个run位置
            self.pos += run_len;
            //run的长度各不相同，需要合并不符合
            //A > B + C 且 B > C 规则的run 
            self.merge_collapse();
        }
        // 不管合并规则，强制从栈顶开始合并剩下的
        // 所有run到只剩下一个run，则tim_sort排序完成
        self.merge_force_collapse();
    }
    //合并run使得A > B + C 且B > C 
    //如果A <= B + C, 则 B 和A、C 中较短的合并
    //如果只有A、B，则A <= B 时 A 和B 合并
    fn merge_collapse(&mut self) {
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n = runs.len() - 2;

            //判断A B C D 的关系，加入D是为了防止特殊情况的Bug
            //A <= B + C || D <= A + B 
            if (n >= 1 && runs[n - 1].len <= runs[n].len + runs[n + 1].len) || (n >= 2 && runs[n - 2].len <= runs[n].len + runs[n - 1].len){
                // 三个连续的 run:A、B、C，判断其长度关系并进行合并
                // n - 1 对应A、n对应B、n + 1 对应C 
                let (pos1, pos2) = if runs[n - 1].len < runs[n + 1].len {
                    (n - 1,n) //A B合并
                } else {
                    (n, n + 1) //B C 合并
                };
                // 取出待合并的run1 和run2
                let (run1, run2) = (runs[pos1], runs[pos2]);
                debug_assert_eq!(run1.pos + run1.len, run2.pos );
                //合并run1 和run2 到run1，其实就是更新run1 的参数并删除run2，
                //run1 下标不变，但合并后长度是run1 和 run2长度之和
                runs.remove(pos2);
                runs[pos1] = Run {
                    pos: run1.pos,
                    len: run1.len + run2.len,
                };
                //取出合并后的run1 去进行归并排序
                let new_list = self.list
                    .split_at_mut(run1.pos).1 
                    .split_at_mut(run1.len + run2.len).0;
                merge_sort(new_list, run1.len, run2.len);
            } else {
                break;
            }
        }
    }
    //集合处理完了就强制合并剩余的run到只剩下一个run 
    fn merge_force_collapse(&mut self) {
        let runs = &mut self.runs;
        while runs.len() > 1 {
            let n = runs.len() - 2;
            //三个连续的run：A、B、C，判断其长度关系并进行合并
            //n - 1 对应A、n对应B、n + 1 对应C 
            let (pos1, pos2) = if n > 0 && runs[n - 1].len < runs[n + 1].len {
                (n - 1, n)
            } else {
                (n, n + 1)
            };
            // 取出待合并分区run1 和 run2
            let (run1, run2) = (runs[pos1], runs[pos2]);
            debug_assert_eq!(run1.len, run2.pos);
            // 合并run1 和run2 到run1 即更新run1 的参数并删除 run2
            // run1下标不变，但合并后长度是run1 和run2长度之和
            runs.remove(pos2);
            runs[pos1] = Run {
                pos: run1.pos,
                len: run1.len + run2.len,
            };
            //取出合并后的run1去进行归并排序
            let new_list = self.list
                .split_at_mut(run1.pos).1 
                .split_at_mut(run1.len + run2.len).0;
            merge_sort(new_list, run1.len, run2.len);
        }
    }
}

//timSort入口
fn tim_sort(list: &mut [i32]) {
    if list.len() < MIN_MERGE {
        binary_insertion_sort(list);
    } else {
        let mut sort_state = SortState::new(list);
        sort_state.sort();
    }
}

fn main(){
    let start = Instant::now();
    let mut nums: Vec<i32> = vec![
         2,  4,  7,  8, 23, 19, 16, 14, 13, 12, 10, 20,
        18, 17, 15, 11,  9, -1,  5,  6,  1,  3, 21, 40,
        22, 39, 38, 37, 36, 35, 34, 33, 24, 30, 31, 32,
        25, 26, 27, 28, 29, 41, 42, 43, 44, 45, 46, 47,
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59,
        60, 80, 79, 78, 77, 76, 75, 74, 73, 72, 71, 70,
        61, 62, 63, 64, 65, 66, 67, 68, 69, 95, 94, 93,
        92, 91, 90, 85, 82, 83, 84, 81, 86, 87, 88, 89,
        321, 923, 754, 231, 635, 893, 110, 367, 451, 555,
        1132, 5532, 8094, 1521, 3306, 6379, 3389, 9521,5432,8396,
    ];
    tim_sort(&mut nums);
    println!("Tim sorted nums: {:?}", nums);
    println!("Time cost: {:?}ms ", start.elapsed().as_millis());
}
