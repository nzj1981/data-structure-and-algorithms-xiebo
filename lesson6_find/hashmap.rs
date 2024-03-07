//hashmap.rs 
//Hashmap的实现
//用slot保存位置，data保存数据， cap控制容量
use std::time::Instant;
#[derive(Debug, Clone, PartialEq)]
struct HashMap <T>{
    cap: usize,
    slot: Vec<usize>,
    data: Vec<T>,
}
impl<T> HashMap<T>
        where T: Clone + PartialEq + Default {
     fn new(cap: usize) -> Self{
         //初始化slot 和 data 
         let mut slot = Vec::with_capacity(cap);
         let mut data = Vec::with_capacity(cap);
         for _i in 0..cap{
             slot.push(0);
             data.push(Default::default());
         }
         HashMap { cap, slot, data }
     }

     fn len(&self) -> usize {
         let mut len = 0;
         for &d in self.slot.iter(){
             //槽中数据不为0，表示有数据，len 加1
             if 0 != d {
                 len += 1;
             }
         }
         len
     }

     fn is_empty(&self) -> bool{
         let mut empty = true;
         for &d in self.slot.iter(){
             if 0 != d {
                 empty = false;
                 break;
             }
         }
         empty
     }

     fn clear(&mut self){
         let mut slot = Vec::with_capacity(self.cap);
         let mut data = Vec::with_capacity(self.cap);
         for _i in 0..self.cap{
             slot.push(0);
             data.push(Default::default());
         }
         self.slot = slot;
         self.data = data;
     }

     fn hash(&self, key: usize) -> usize {
         key % self.cap
     }

     fn rehash(&self, pos: usize) -> usize{
         (pos + 1) % self.cap
     }
    
     fn insert(&mut self, key: usize, value: T){
         if 0 == key { panic!("Error: key must > 0");}
         let pos = self.hash(key);
         if 0 == self.slot[pos]{
             //槽无数据直接插入
             self.slot[pos] = key;
             self.data[pos] = value;
         } else {
             //插入有数据再找下一个可行的位置
             let mut next = self.rehash(pos);
             while 0 != self.slot[next] && key != self.slot[next] {
                 next = self.rehash(next);
                 //槽满了就退出
                 if next == pos {
                     println!("Error: slot is full!");
                     return;
                 }
             }
             //在找到的槽插入数据
             if 0 == self.slot[next]{
                 self.slot[next] = key;
                 self.data[next] = value;
             } else {
                 self.data[next] = value;
             }
         }
     }

}


fn main(){
    let start = Instant::now();
    basic();

    println!("Time cost: {}ms", start.elapsed().as_millis());
}
fn basic(){
    let mut hmap = HashMap::new(11);
    hmap.insert(2, "dog");
    hmap.insert(3, "tiger");
    hmap.insert(10, "cat");
    println!("{:?}", hmap);
}
