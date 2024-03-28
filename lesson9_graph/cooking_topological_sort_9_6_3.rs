//烹饪的拓扑排序
use std::time::Instant;
use std::collections::HashMap;
use std::hash::Hash;
use std::fmt::Display;

//颜色枚举
#[derive(Debug, Clone, PartialEq)]
enum Color {
    White, //白色,未被探索
    Gray, //灰色正被探索
    Black, //黑色，已被探索
}

//烹饪流程节点定义 
#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    color: Color,
    neighbors: Vec<T>,
}
impl<T> Vertex<T>
    where T:PartialEq + Clone {
    fn new(key: T) -> Self {
        Self {
            key: key,
            color: Color::White,
            neighbors: Vec::new(),
        }
    }
    fn add_neighbor(&mut self, nbr: T) {
        self.neighbors.push(nbr);
    }
}

//烹饪关系图定义
#[derive(Debug, Clone)]
struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>, 
    edges: HashMap<T, Vec<T>>,
}
impl<T> Graph<T>
    where T: Eq + PartialEq + Clone + Hash
{
    fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
            edges: HashMap::<T, Vec<T>>::new(),
        }
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }
    
    fn add_edge(&mut self, src: &T, des: &T) {
        if !self.vertices.contains_key(src) { let _sv = self.add_vertex(src);}
        if !self.vertices.contains_key(des) { let _dv = self.add_vertex(des);}
        //添加点
        self.edgenums += 1;
        self.vertices.get_mut(src)
                     .unwrap()
                     .add_neighbor(des.clone());
        //添加边
        if !self.edges.contains_key(src) {
            let _eg = self.edges.insert(src.clone(), Vec::new());
        }
        self.edges.get_mut(src).unwrap().push(des.clone());
    }
}

//构建烹饪关系图
fn build_cooking_graph<T>(pre_requisties: Vec<Vec<T>>) -> Graph<T>
    where T: Eq + PartialEq + Clone + Hash
{
    //为依赖的烹饪创建关系
    let mut cooking_graph = Graph::new();
    for v in pre_requisties.iter(){
        let prev = v.first().unwrap();
        let last = v.last().unwrap();
        cooking_graph.add_edge(prev, last);
    }
    cooking_graph
}
//烹饪规划
fn cooking_scheduling<T> (
    cg: &mut Graph<T>,
    cook: Vertex<T>,
    schedule: &mut Vec<String>,
    mut has_circle: bool)
    where T: Eq + PartialEq + Clone + Hash + Display
{
    //克隆，防止可变引用冲突
    let edges = cg.edges.clone();
    //对依赖烹饪进行探索
    let dependencies = edges.get(&cook.key);
    if  !dependencies.is_none() {
        for dependency in dependencies.unwrap().iter() {
            let cook = cg.vertices.get(dependency).unwrap().clone();
            if Color::White == cook.color {
                cg.vertices.get_mut(dependency).unwrap().color = Color::Gray;
                cooking_scheduling(cg, cook, schedule, has_circle);
                if has_circle {
                    return;
                }
            } else if Color::Gray == cook.color {
                has_circle = true;
                return;
            }
        }
    } 
    //修改烹饪节点颜色,表示当前烹饪课程节点探索完成 加入schedule
    cg.vertices.get_mut(&cook.key).unwrap().color = Color::Black;
    schedule.push(cook.key.to_string());
}

fn find_topological_order<T>(cook_num: usize, pre_requisties: Vec<Vec<T>>)
    where T: Eq + PartialEq + Clone + Hash + Display
{
    //构建烹饪关系图
    let mut cg = build_cooking_graph(pre_requisties);
    //获取所有的课程节点到cook 
    let vertices = cg.vertices.clone();
    let mut cooks = Vec::new();
    for key in vertices.keys() {
        cooks.push(key);
    }
    //对烹饪过程进行拓扑排序
    //保存可行的烹饪流程
    let mut schedule = Vec::new();
    let has_circle = false;
    for i in 0..cook_num {
        let cook = cg.vertices.get(&cooks[i]).unwrap().clone();
        if !has_circle && Color::White == cook.color {
            cg.vertices.get_mut(&cooks[i]).unwrap().color = Color::Gray;
            cooking_scheduling(&mut cg, cook, &mut schedule, has_circle);
        }
    }
    if !has_circle {
        println!("{:#?}", schedule);
    }
    
}

fn main(){
    let s_time = Instant::now();
    let operation_num = 9;
    //构建做菜顺序依赖关系
    let mut pre_requisties = Vec::<Vec<&str>>::new();
    pre_requisties.push(vec!["混合", "3/4杯牛奶"]);
    pre_requisties.push(vec!["混合", "一个鸡蛋"]);
    pre_requisties.push(vec!["混合", "一勺橄榄油"]);
    pre_requisties.push(vec!["倒入1/4杯", "混合"]);
    pre_requisties.push(vec!["倒入1/4杯", "加热锅"]);
    pre_requisties.push(vec!["底面金黄翻面", "倒入1/4"]);
    pre_requisties.push(vec!["享用", "底面金黄翻面"]);
    pre_requisties.push(vec!["享用", "加热糖浆"]);
    //找到拓扑排序结果，即合理的做菜顺序
    find_topological_order(operation_num, pre_requisties);

    println!("Time cost: {:?}ms", s_time.elapsed().as_millis());
}
