//hot_potato.rs
//烫手的山芋游戏或叫击鼓传花
pub mod queue_mod;
use crate::queue_mod::Queue;

fn hot_potato(names: Vec<&str>, num:usize) -> &str {
    //初始化队列、名字入队
    let mut q = Queue::new(names.len());
    for name in names{
        let _nm = q.enqueue(name);
    }

    while q.len() > 1 {
        //出入栈名字，相当于传递山芋
        for _i in 0..num {
            let name = q.dequeue().unwrap();
            let _rm = q.enqueue(name);
        }
        //出入栈到num次，删除一人
        let _rm = q.dequeue();
    }
    //最后剩下的人
    q.dequeue().unwrap()
}

fn main(){
    let names = vec!["Mon", "Tom", "Kew", "Lisa", "Marry", "Bob", "Autumner"];
    let survivor = hot_potato(names, 8);
    println!("The survivor person is {survivor}");
}
