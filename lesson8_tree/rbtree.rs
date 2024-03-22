//rbtree.rs 
//红黑树
use std::time::Instant;
use std::boxed::Box;
use std::cmp::{Ord, max, Ordering::*};
use std::fmt::Debug;
use std::iter::Iterator;
use std::ptr::null_mut;

//此队列用于层序遍历
#[derive(Debug)]
struct Queue<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Queue<T> {
    fn new(size: usize) -> Self {
        Self {
            cap: size,
            data: Vec::with_capacity(size),
        }
    }

    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if Self::len(&self) == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    fn dequeue(&mut self) -> Option<T> {
        if self.len() > 0 {
            self.data.pop()
        } else {
            None
        }
    }
    fn is_empty(&self) -> bool {
        0 == self.len()
    }
    fn len(&self) -> usize {
        self.data.len()
    }
}
//红黑树颜色枚举
#[derive(Debug)]
enum Color {
    Red,
    Black,
}

//红黑树节点定义
struct RBNode<K: Ord + Debug, V> {
    key: K,
    val: V,
    color: Color,
    parent: *mut RBNode<K, V>,
    left: *mut RBNode<K, V>,
    right: *mut RBNode<K, V>,
}
impl<K: Ord + Debug, V> RBNode<K, V> {
    fn new(key: K, val: V) -> Self {
        Self {
            key, 
            val,
            color: Color::Red,
            parent: null_mut(),
            left: null_mut(),
            right: null_mut(),
        }
    }
    unsafe fn size(&self, mut size: usize) -> usize {
        size += 1;
        if !self.left.is_null() {
            size = (*self.left).size(size);
        }
        if !self.right.is_null() {
            size = (*self.right).size(size);
        }
        size
    }
    unsafe fn leaf_size(&self) -> usize {
        if self.left.is_null() && self.right.is_null() {
            return 1;
        }

        let mut left_leaf = 0;
        if !self.left.is_null() {
            left_leaf = (*self.left).leaf_size();
        }
        
        let mut right_leaf = 0;
        if !self.right.is_null() {
            right_leaf = (*self.right).leaf_size();
        }

        left_leaf + right_leaf
    }
    unsafe fn none_leaf_size(&self) -> usize {
        self.size(0) - self.leaf_size()
    }
    unsafe fn depth(&self) -> usize {
        let mut dl = 1;
        if !self.left.is_null() {
            dl += (*self.left).depth();
        }

        let mut dr = 1;
        if !self.right.is_null(){
            dr += (*self.right).depth();
        }

        max(dl, dr)
    }

    unsafe fn min(&self) -> Option<(&K, &V)> {
        if self.left.is_null() {
            Some((&self.key, &self.val))
        } else {
            (*self.left).min()
        }
    }

    unsafe fn max(&self) -> Option<(&K, &V)> {
        if self.right.is_null(){
            Some((&self.key, &self.val))
        } else {
            (*self.right).max()
        }
    }

    unsafe fn contains(&self, key: &K) -> bool {
        match self.key.cmp(key) {
            Equal => true,
            Greater => {
                if !self.left.is_null() {
                    (*self.left).contains(key)
                } else {
                    false
                }
            },
            Less => {
                if !self.right.is_null() {
                    (*self.right).contains(key)
                } else {
                    false
                }
            },
        }
    }

    //红黑树前中后排序内部实现
    unsafe fn preorder(&self) {
        println!("Internal preorder key: {:?}", &self.key);
        if !self.left.is_null() {(*self.left).preorder();}
        if !self.right.is_null() {(*self.right).preorder();}
    }
    unsafe fn inorder(&self) {
        if !self.left.is_null() {(*self.left).inorder();}
        println!("Internal inorder key: {:?}", &self.key);
        if !self.right.is_null() {(*self.right).inorder();}
    }
    unsafe fn postorder(&self) {
        if !self.left.is_null(){(*self.left).postorder();}
        if !self.right.is_null(){(*self.right).postorder();}
        println!("Internal postorder key: {:?}", &self.key);
    }
    unsafe fn levelorder(&self) {
        let size = self.size(0);
        let mut q = Queue::new(size);

        let _r = q.enqueue(self.clone());
        while !q.is_empty(){
            let front = q.dequeue().unwrap();
            println!("Internal levelorder key: {:?}", front.key);
            if !front.left.is_null() {
                let _r = q.enqueue(&(*front.left));
            }
            if !front.right.is_null() {
                let _r = q.enqueue(&(*front.right));
            }
        }
    }
}
//红黑树定义
struct RBTree<K: Ord + Debug, V> {
    root: *mut RBNode<K, V>,
}
//为红黑树实现默认值
impl<K: Ord + Debug, V> Default for RBTree<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
//实现红黑树
impl<K: Ord + Debug, V> RBTree<K, V> {
    fn new() -> Self {
        Self { root: null_mut()}
    }
    fn is_empty(&self) -> bool {
        self.root.is_null()
    }
    //计算树节点数
    fn size(&self) -> usize {
        unsafe {
            (*self.root).size(0)
        }
    }
    //计算叶节点数
    fn leaf_size(&self) -> usize {
        unsafe {
            (*self.root).leaf_size()
        }
    }
    // 计算非叶节点数
    fn none_leaf_size(&self) -> usize {
        unsafe {
            (*self.root).none_leaf_size()
        }
    }
    //树深度
    fn depth(&self) -> usize {
        unsafe {
            (*self.root).depth()
        }
    }
    //最大最小值
    fn min(&self) -> Option<(&K, &V)> {
        unsafe {
            (*self.root).min()
        }
    }
    fn max(&self) -> Option<&K, &V> {
        unsafe {
            (*self.root).max()
        }
    }
    
    //数据查询
    fn contains(&self, key: &K) -> bool {
        unsafe {
            (*self.root).contains(key)
        }
    }
    //获取值引用及可变引用
    fn get(&self, key: &K) -> Option<&V> {
        unsafe {
            let mut node = self.root;
            while !node.is_null() {
                node = match (*node).key.cmp(key) {
                    Less => (*node).right,
                    Equal => return Some(&(*node).val),
                    Greater => (*node).left,
                }
            }
        }
        None
    }
    fn get_mut(&self, key: &K) -> Option<&mut V> {
        unsafe {
            let mut node = self.root;
            while !node.is_null() {
                node = match (*node).key.cmp(key) {
                    Less => (*node).right,
                    Equal => return Some(&mut (*node).val),
                    Greater => (*node).left,
                }
            }
        }
        None
    }
    //数据插入
    fn insert(&mut self, key: K, val: V) {
        unsafe {
            let mut parent = null_mut();
            let mut node = self.root;

            //找到待插入节点及其父节点位置
            while !node.is_null() {
                parent = node;
                node = match (*node).key.cmp(&key) {
                    Less => (*node).right,
                    Equal => {
                        (*node).val = val;
                        return;
                    }
                    Greater => (*node).left,
                }
            }
        //数据插入
        }
    }
}

fn main(){
    let start = Instant::now();

    println!("Time cost: {:?}", start.elapsed().as_millis());
}
