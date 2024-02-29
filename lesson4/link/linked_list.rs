//linked_list.rs 
//链表创建

//节点链接用Box指针(大小确定),因为确定大小才能分配内存
type Link<T> = Option<Box<Node<T>>>;
//链表定义
struct List<T> {
    size: usize,  //链表节点数
    head: Link<T>, //头节点
}

//链表节点
struct Node<T> {
    elem: T, //数据
    next: Link<T>, //下一个节点链接
}

impl<T> List<T> {
    fn new()-> Self{
        Self{
            size: 0,
            head: None
        }
    }
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    fn len(&self) -> usize{
        self.size
    }
    fn clear(&mut self) {
        self.size = 0;
        self.head = None;
    }
    //新节点总是加到头部
    fn push(&mut self, elem: T){
        let node = Box::new(Node{
            elem: elem,
            next: self.head.take(),
        });
        self.head = Some(node);
        self.size += 1;
    }
    //take会取出数据留下空位
    fn pop (&mut self) -> Option<T> {
        self.head.take().map(|node|{
            self.head = node.next;
            self.size -= 1;
            node.elem
        })
    }
    //peek不改变值，只能引用
    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.elem)
    }

    //peek_mut可改变值，是可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.elem)
    }

    /*
    *以下是为链表实现的迭代功能 
    *into_iter：链表改变，成为迭代器
    *iter: 链表不变，只是得到不可变迭代器
    *iter_mut:链表不变，得到可变迭代器
    */
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
    fn iter(&self) -> Iter<T> {
        Iter { next: self.head.as_deref()}
    }
    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut { next: self.head.as_deref_mut()}
    }
}

//实现三种迭代功能 
struct IntoIter<T>(List<T>);
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop() //(List<T>)元组的第0项
    }
}

struct Iter<'a, T: 'a> {next: Option<&'a Node<T>>}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node|{
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

struct IterMut<'a, T: 'a>{next: Option<&'a mut Node<T>>}
impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item>{
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.elem
        })
    }
}

//为链表实现自定义Drop
impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut link = self.head.take();
        while let Some(mut node) = link {
            link = node.next.take();
        }
    }
}

fn main(){
    basic_test();
    into_iter_test();
    iter_test();
    iter_mut_test();
}

fn basic_test(){
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    assert_eq!(list.len(),3 );
    assert_eq!(false, list.is_empty() );
    assert_eq!(Some(3),list.pop());
    assert_eq!(Some(&2),list.peek());
    assert_eq!(Some(&mut 2),list.peek_mut());

    list.peek_mut().map(|val|{
        *val = 4;
    });
    assert_eq!(Some(&4),list.peek() );
    list.clear();
    println!("basics test Ok!");
}
fn into_iter_test(){
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.into_iter();
    assert_eq!(Some(3),iter.next() );
    assert_eq!(Some(2),iter.next() );
    assert_eq!(Some(1),iter.next() );
    assert_eq!(None,iter.next() );
    println!("into_iter test Ok!");
}
fn iter_test(){
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter();
    assert_eq!(Some(&3),iter.next() );
    assert_eq!(Some(&2),iter.next() );
    assert_eq!(Some(&1),iter.next() );
    assert_eq!(None,iter.next() );
    println!("iter test Ok!");
}
fn iter_mut_test(){
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    let mut iter = list.iter_mut();
    assert_eq!(Some(&mut 3),iter.next() );
    assert_eq!(Some(&mut 2),iter.next() );
    assert_eq!(Some(&mut 1),iter.next() );
    assert_eq!(None,iter.next() );
    println!("iter_mut test Ok!");
}
