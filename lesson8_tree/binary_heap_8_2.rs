//binary_heap_8_2.rs 
//二叉堆
use std::time::Instant;
//计算父节点及左右子节点下标
macro_rules! parent {
    ($child: ident) => ($child >> 1)
}
macro_rules! left_child {
    ($parent:ident) => ($parent << 1)
}
macro_rules! right_child {
    ($parent:ident) => (($parent << 1) + 1)
}

//二叉堆定义
#[derive(Debug, Clone)]
struct BinaryHeap {
    size: usize, //数据量
    data: Vec<i32>, //数据容器
}
impl BinaryHeap {
    fn new() -> Self {
        Self {
            size: 0, //vec 首位置 0，但不计入总数
            data: vec![0], //动态数组初始是0
        }
    }
    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        0 == self.size
    }

    //获取堆中最小数据
    fn min(&self) -> Option<i32> {
        if 0 == self.size {
            None
        } else {
            //若数据类型是泛型，则用clone 
            //Some(self.data[1].clone());
            Some(self.data[1])
        }
    }
    //切片数据逐个加入堆
    fn build_add(&mut self, arr: &[i32]) {
        for &val in arr {
            self.push(val);
        }
    }

    //末尾添加一个数据，调整堆
    fn push(&mut self, val: i32){
        self.data.push(val);
        self.size += 1;
        self.move_up(self.size);
    }

    // 构建新堆
    fn build_new(&mut self, arr: &[i32]){
        // 删除原始数据
        for _i in 0..self.size {
            let _rm = self.data.pop();
        }
        //添加新数据
        for &val in arr {
            self.data.push(val);
        }
        self.size = arr.len();
        //调整堆，使其为小顶堆
        let size = self.size;
        let mut p = parent!(size);
        while p > 0 {
            
            p -= 1;
        }
    }
    //弹出最小值
    fn pop(&mut self) -> Option<i32> {
        if 0 == self.size {
            None
        } else if 1 == self.size {
            self.size -= 1;
            self.data.pop()
        } else {
            //多个数据，先交换并弹出数据，再调整堆
            self.data.swap(1, self.size);
            let val = self.data.pop();
            self.size -= 1;
            self.move_down(1);
            val
        }
    }
    // 小数据上冒c(child),p(parent)
    fn move_up(&mut self, mut c: usize) {
        loop {
            //计算当前节点的父节点位置
            let p = parent!(c);
            if p <= 0 {break;}
            //当前节点数据小于父节点数据，交换
            if self.data[c] < self.data[p] {
                self.data.swap(c, p);
            }
            //父节点成为当前节点
            c = p;
        }
    }
    //大数据下沉l(left), r(right)
    fn move_down(&mut self, mut c: usize) {
        loop {
            //计算当前节点的左子节点位置
            let lc = left_child!(c);
            if lc > self.size { break;}

            //计算当前节点最小子节点位置
            let mc = self.min_child(c);

            //当前节点数据大于最小子节点数据，交换
            if self.data[c] > self.data[mc] {
                self.data.swap(c, mc);
            }
            //最小子节点成为当前节点
            c = mc;
        }
    }
    // 计算最小子节点位置
    fn min_child(&self, c: usize) -> usize {
        //同时计算左右子节点位置
        let (lc, rc) = (left_child!(c), right_child!(c));
        /*
         *1.如果右子节点位置超过size，表示只有左子点
         *则左子节点就是最小子节点
         *2.否则，同时存在左右子节点，需具体判断左右
         *子节点数据大小，然后返回最小的子节点位置
         */
        if rc > self.size {
            lc 
        } else if self.data[rc] > self.data[lc] {
            lc 
        } else {
            rc 
        }
    }
}

fn main() {
    let start = Instant::now();
    let mut bh = BinaryHeap::new();
    let nums = [-1, -1, 2, 3, 4];
    bh.push(10);
    bh.push(9);
    bh.push(8);
    bh.push(7);
    bh.push(6);

    println!("Struct bh:{:#?}", bh);
    bh.build_add(&nums);
    println!("empty: {:?}", bh.is_empty());
    println!("min: {:?}", bh.min());
    println!("size: {:?}", bh.size());
    println!("Struct bh:{:#?}", bh);
    println!("pop min: {:?}", bh.pop());

    bh.build_new(&nums);
    println!("size: {:?}", bh.size());
    println!("pop min: {:?}", bh.pop());
    println!("Struct bh:{:#?}", bh);
    println!("Time cost:{:?}ms", start.elapsed().as_millis());
}

