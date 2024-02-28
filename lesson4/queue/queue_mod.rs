//创建一个队列结构体以及行为方法，可被其它文件调用
#[derive(Debug)]
pub struct Queue<T>{
    cap: usize,
    data: Vec<T>, 
}
impl<T> Queue<T> {
    pub fn new(size: usize) -> Self {
        Self{
            cap: size,
            data: Vec::with_capacity(size),
        }
    }
    pub fn enqueue(&mut self, val:T) -> Result<(), String>{
        if self.len() == self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    pub fn dequeue(&mut self) -> Option<T> {
        if self.len()>0{
            self.data.pop()
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

