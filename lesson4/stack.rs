//rust 创建栈
#[derive(Debug)]
pub struct Stack<T> {
    size: usize, //栈大小
    data: Vec<T>, //栈数据
}

impl<T> Stack<T> {
    //初始化空栈
  pub fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new()
        }
    }

  pub fn is_empty(&self) -> bool {
        0 == self.size
    }

   pub fn len(&self) -> usize {
        self.size
    }

    //清空栈
    pub fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    //将数据保存在Vec的末尾
   pub fn push(&mut self, val:T) {
        self.data.push(val);
        self.size += 1;
    }

    //在将栈顶减1后，弹出数据
    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.size{return None;}
        self.size -= 1;
        self.data.pop()
    }

    //返回栈顶数据引用和可变引用
    pub fn peek(&self) -> Option<&T> {
        if 0 == self.size {return None;}
        self.data.get(self.size - 1)
    }
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size {return None;}
        self.data.get_mut(self.size - 1)
    }

    //以下是为栈实现的迭代功能
    //into_iter:栈改变，成为迭代器
    //iter:栈不变，得到不可变迭代器
    //iter_mut:栈不变，得到可变迭代器
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    pub fn iter(&self) -> Iter<T> {
        let mut iterator = Iter{stack: Vec::new()};
        for item in self.data.iter(){
            iterator.stack.push(item);
        }
        iterator
    }
    pub fn iter_mut(&mut self)-> IterMut<T> {
        let mut iterator = IterMut{stack: Vec::new()};
        for item in self.data.iter_mut(){
            iterator.stack.push(item);
        }
        iterator
    }
}


//实现三种迭代功能
pub struct IntoIter<T>(Stack<T>);
impl<T:Clone> Iterator for IntoIter<T> {
    // add code here
    type Item = T;
    fn next(&mut self)->Option<Self::Item>{
        if !self.0.is_empty(){
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}


pub struct Iter<'a, T:'a>{stack: Vec<&'a T>,}
impl<'a,T> Iterator for Iter<'a,T>{
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item>{
        self.stack.pop()
    }
}

pub struct IterMut<'a, T: 'a>{stack:Vec<&'a mut T>}
impl<'a, T> Iterator for IterMut<'a, T>{
    type Item = &'a mut T;
    fn next(&mut self)->Option<Self::Item>{
        self.stack.pop()
    }
}
