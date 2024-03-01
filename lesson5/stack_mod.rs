//stack_mod.rs 
#[derive(Debug)]
pub struct Stack<T> {
    top: usize,
    data: Vec<T>,
}
impl<T> Stack<T>{
    pub fn new()->Self{
        Self{
            top: 0,
            data: Vec::new()
        }
    }
    pub fn push(&mut self, val:T){
        self.data.push(val);
        self.top += 1;
    }
    pub fn pop(&mut self) -> Option<T> {
        if 0 == self.top{return None;}
        self.top -= 1;
        self.data.pop()
    }
    pub fn is_empty(&self) -> bool {
        0 == self.top
    }
}
