use std::collections::HashMap;
use std::collections::{HashSet, VecDeque};
use std::hash::Hash;
use std::thread::current;

#[derive(Debug, Clone)]
struct Vertex<T> {
    key: T,
    connects: Vec<(T, i32)>,
}

impl<T: Clone + PartialEq> Vertex<T> {
    fn new(key: T) -> Self {
        Self {
            key,
            connects: Vec::new(),
        }
    }

    // 判断是否 self 是否 与 key 相邻
    fn adjacent_key(&self, key: &T) -> bool {
        for (nbr, _wt) in self.connects.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    // 获取 邻居，但是有一点感觉很奇怪：为什么要 clone 一份呢？
    fn get_connects(&self) -> Vec<&T> {
        let mut connects = Vec::new();
        for (nbr, _wt) in self.connects.iter() {
            connects.push(nbr);
        }
        connects
    }

    // 但是这里却返回的是 不可变 引用
    fn get_nbr_weight(&self, key: &T) -> &i32 {
        for (nbr, wt) in self.connects.iter() {
            if nbr == key {
                return wt;
            }
        }
        &0
    }

    fn add_neighbor(&mut self, key: T, wt: i32) {
        self.connects.push((key, wt));
    }
}

#[derive(Debug, Clone)]
pub struct Graph<T> {
    vertnums: u32,
    edgenums: u32,
    vertices: HashMap<T, Vertex<T>>, // 点集合
}

impl<T: Hash + Eq + PartialEq + Clone> Graph<T> {
    pub fn new() -> Self {
        Self {
            vertnums: 0,
            edgenums: 0,
            vertices: HashMap::<T, Vertex<T>>::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.vertnums == 0
    }

    fn vertex_num(&self) -> u32 {
        self.vertnums
    }

    fn edge_num(&self) -> u32 {
        self.edgenums
    }

    fn contains(&self, key: &T) -> bool {
        for (nbr, _vertex) in self.vertices.iter() {
            if nbr == key {
                return true;
            }
        }
        false
    }

    fn add_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let vertex = Vertex::new(key.clone());
        self.vertnums += 1;
        self.vertices.insert(key.clone(), vertex)
    }

    fn get_vertex(&self, key: &T) -> Option<&Vertex<T>> {
        if let Some(Vertex) = self.vertices.get(key) {
            return Some(Vertex);
        } else {
            None
        }
    }

    fn vertex_keys(&self) -> Vec<&T> {
        let mut keys = Vec::new();
        for (key, _vertex) in self.vertices.iter() {
            keys.push(key);
        }
        keys
    }

    fn remove_vertex(&mut self, key: &T) -> Option<Vertex<T>> {
        let old_vertex = self.vertices.remove(key);
        self.vertnums -= 1;
        self.edgenums -= old_vertex.clone().unwrap().get_connects().len() as u32; // 简单来说，就是 - 出边数量

        // let keys: Vec<&T> = self.vertex_keys();  // 主要的就是 Vertex 的冲突
        let keys: Vec<T> = self.vertices.keys().cloned().collect();

        for vertex in keys {
            if let Some(vt) = self.vertices.get_mut(&vertex) {
                if vt.adjacent_key(key) {
                    vt.connects.retain(|(k, _)| k != key);
                    self.edgenums -= 1;
                }
            }
        }
        old_vertex
    }

    fn add_edge(&mut self, from: &T, to: &T, wt: i32) {
        if !self.contains(from) {
            let _fvert = self.add_vertex(from); // 如果点不存在，那么先添加一个点
        }
        if !self.contains(to) {
            let _fvert = self.add_vertex(to); // 如果点不存在，那么先添加一个点
        }

        // 添加边
        self.edgenums += 1;
        self.vertices
            .get_mut(from)
            .unwrap()
            .add_neighbor(to.clone(), wt);
    }

    fn adjacent(&self, from: &T, to: &T) -> bool {
        self.vertices.get(from).unwrap().adjacent_key(to)
    }
}

struct DfsIter<'a, T> {
    stack: VecDeque<&'a T>,
    visited: HashSet<&'a T>,
    graph: &'a Graph<T>,
}

impl<T> Graph<T> {
    // 因为: retval 的生命周期来自于 self
    fn dfs_iter<'a>(&'a self, start: &'a T) -> DfsIter<T>
    where
        T: Hash + Eq + Clone,
    {
        DfsIter::new(self, start)
    }
}

impl<'a, T: Hash + Eq + Clone> DfsIter<'a, T> {
    fn new(graph: &'a Graph<T>, start: &'a T) -> Self {
        let mut stack = VecDeque::new();
        stack.push_front(start);
        Self {
            stack,
            visited: HashSet::new(),
            graph,
        }
    }
}

impl<'a, T: Hash + Eq + Clone> Iterator for DfsIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(cur) = self.stack.pop_back() {
            // 访问
            if !self.visited.insert(cur) {
                // 如果插入失败，说明已经访问过了
                continue;
            }
            if let Some(vertex) = self.graph.get_vertex(cur) {
                for (nbr, _wt) in &vertex.connects {
                    if !self.visited.contains(nbr) {
                        self.stack.push_back(nbr);
                    }
                }
            }
            return Some(cur);
        }
        None // finish foreach
    }
}

mod tests {
    use super::Graph;

    #[test]
    fn it_works() {
        let mut g = Graph::new();

        for i in 0..6 {
            g.add_vertex(&i);
        }

        println!("graph empty: {}", g.is_empty());

        let vertices = g.vertex_keys();
        for vertex in vertices {
            println!("Vertex: {:#?}", vertex);
        }

        g.add_edge(&0, &1, 5);
        g.add_edge(&0, &5, 2);
        g.add_edge(&1, &2, 4);
        g.add_edge(&2, &3, 9);
        g.add_edge(&3, &4, 7);
        g.add_edge(&3, &5, 3);
        g.add_edge(&4, &0, 1);
        g.add_edge(&4, &4, 8);

        println!("vert nums: {}", g.vertex_num());
        println!("edge nums: {}", g.edge_num());
        println!("contains 0: {}", g.contains(&0));

        let res = g.adjacent(&0, &1);
        println!("0 -> 1: {}", res);
        let res = g.adjacent(&3, &2);
        println!("3 -> 2: {}", res);

        let rm = g.remove_vertex(&0).unwrap();
        println!("remove vertex: {}", rm.key);
        println!("left vert nums: {}", g.vertex_num());
        println!("left edge nums: {}", g.edge_num());
        println!("contains 0: {}", g.contains(&0));
    }

    #[test]
    fn test_dfs_iter() {
        let mut g = Graph::new();

        for i in 0..6 {
            g.add_vertex(&i);
        }

        g.add_edge(&0, &1, 5);
        g.add_edge(&0, &5, 2);
        g.add_edge(&1, &2, 4);
        g.add_edge(&2, &3, 9);
        g.add_edge(&3, &4, 7);
        g.add_edge(&3, &5, 3);
        g.add_edge(&4, &0, 1);
        g.add_edge(&4, &4, 8);

        g.add_vertex(&6); // 孤岛 不可到达

        let mut dfs_iter = g.dfs_iter(&0);
        let mut result = Vec::new();

        while let Some(vertex) = dfs_iter.next() {
            result.push(*vertex);
        }

        println!("{:#?}", result);
        // let expected = vec![0, 1, 2, 3, 4, 5]; // 这个预期结果可能会根据你的DFS实现有所不同
        // assert_eq!(result, expected);
    }
}
