//队列的mod
//deque_mod.rs 
#[derive(Debug)]
pub struct Deque<T> {
    cap: usize,
    data: Vec<T>,
}

impl<T> Deque<T> {
    pub fn new(cap: usize) -> Self {
        Self {
            cap: cap,
            data: Vec::with_capacity(cap),
        }
    }

    pub fn add_rear(&mut self, val: T) -> Result<(), String>{
        if self.size() == self.cap {
            return Err("No space avaliable".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }

    pub fn remove_front(&mut self) -> Option<T> {
        if self.size() > 0 {
            self.data.pop()
        }else {
            None
        }
    }

    pub fn remove_rear(&mut self) -> Option<T> {
        if self.size() > 0 {
            Some(self.data.remove(0))
        }else {
            None
        }
    }
    pub fn size(&self) -> usize {
        self.data.len()
    }
}
