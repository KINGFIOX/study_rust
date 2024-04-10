use std::process::id;

/// 邻接矩阵

/// 点的定义
#[derive(Debug)]
struct Vertex<'a> {
    id: usize,
    name: &'a str,
}

impl Vertex<'_> {
    fn new(id: usize, name: &str) -> Vertex {
        Vertex { id, name }
    }
}

/// 边的定义
#[derive(Debug, Clone)]
struct Edge {
    edge: bool,
}

impl Edge {
    fn new() -> Self {
        Self { edge: false }
    }

    fn set_edge() -> Self {
        Self { edge: true }
    }
}

/// 图的定义
#[derive(Debug)]
struct Graph {
    nodes: usize,
    graph: Vec<Vec<Edge>>,
}

impl Graph {
    fn new(nodes: usize) -> Self {
        Self {
            nodes,
            // vec![item; times] 这是创建 times 个 item 对象。创建的时候，内部调用了 item.clone() 方法
            graph: vec![vec![Edge::new(); nodes]; nodes],
        }
    }

    fn len(&self) -> usize {
        self.nodes
    }

    fn is_empty(&self) -> bool {
        self.nodes == 0
    }

    fn add_edge(&mut self, n1: &Vertex, n2: &Vertex) {
        if n1.id < self.nodes && n2.id < self.nodes {
            self.graph[n1.id][n2.id] = Edge::set_edge();
        } else {
            panic!("Vertex out of range");
        }
    }
}
