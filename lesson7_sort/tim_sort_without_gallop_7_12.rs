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
    first_pen: usize, //run1的长度
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
        }
    }
}

fn main(){
    let start = Instant::now();
    println!("Time cost: {:?}ms ", start.elapsed().as_millis());
}
