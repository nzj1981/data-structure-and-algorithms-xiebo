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

fn main(){
    let start = Instant::now();

    println!("Time cost: {:?}", start.elapsed().as_millis());
}
