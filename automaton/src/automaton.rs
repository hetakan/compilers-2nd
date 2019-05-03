pub type State = Node;
pub type NFA<'a> = Graph<'a>;

#[derive(Debug)]
pub struct Node {
    v: i32
}
impl Node {
    pub fn spawn(n: usize, nodes: &mut Vec<Node>) {
        for i in 0..n {
            nodes.push(Node { v: i as i32 } );
        }
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.v == other.v
    }
}

#[derive(Debug)]
pub struct Edge<'a> {
    head: &'a Node,
    tail: &'a Node,
    label: String,
    epsilon: bool,
}
impl<'a> Edge<'a> {
    fn spawn_inner(node1: &'a State, node2: &'a State, name: &str, epsilon: bool, edges: &mut Vec<Edge<'a>>) {
        edges.push(Edge{ head: node1, tail: node2, label: String::from(name), epsilon: epsilon});
    }
    pub fn spawn(node1: &'a State, node2: &'a State, name: &str, edges: &mut Vec<Edge<'a>>) {
        if node1.v > node2.v {
            Edge::spawn_inner(node2, node1, name, false, edges);
        } else {
            Edge::spawn_inner(node1, node2, name, false, edges);
        }
    }
    pub fn spawn_loop(node: &'a Node, name: &str, edges: &mut Vec<Edge<'a>>) {
        Edge::spawn_inner(node, node, name, false, edges);
    } 
    pub fn spawn_epsilon(node1: &'a State, node2: &'a State, edges: &mut Vec<Edge<'a>>) {
        Edge::spawn_inner(node1, node2, "epsilon", true, edges);
    }
    pub fn is_epsilon(&self) -> bool {
        self.epsilon
    }
    pub fn is_loop(&self) -> bool {
        self.head.v == self.tail.v
    }
}

#[derive(Debug)]
pub struct Graph<'a> {
    states: &'a Vec<Node>,
    edges: &'a Vec<Edge<'a>>,
}
impl<'a> Graph<'a> {
    pub fn make(states: &'a Vec<State>, edges: &'a Vec<Edge<'a>>) -> Graph<'a> {
        Graph {
            states: states, edges: edges,
        }
    }
    pub fn epsilon_nearest_closure(&self, s: &State) -> Vec<&State> {
        self.edges
            .into_iter()
            .filter(|&edge| edge.is_epsilon() && *s == *edge.head)
            .map(|edge| edge.tail)
            .collect()
    }
}

#[test]
fn epsilon_nearest_closure_test() {
    let mut states = vec![];
    Node::spawn(2, &mut states);
    let edges = vec![Edge{head: &states[0], tail: &states[1], epsilon: true, label: String::from("epsilon")}];
    let g = Graph::make(&states, &edges);
    let iter = g.epsilon_nearest_closure(&Node{v: 0});
    assert_eq!(*iter[0], Node{v: 1});
}

#[test]
fn epsilon_nearest_closure_nothing_test() {
    let mut states = vec![];
    Node::spawn(2, &mut states);
    let edges = vec![Edge{head: &states[0], tail: &states[1], epsilon: false, label: String::from("epsilon")}];
    let g = Graph::make(&states, &edges);
    let iter = g.epsilon_nearest_closure(&Node{v: 0});
    assert!(iter.is_empty());
}

#[test]
fn graph_make_test() {
    let mut states = vec![];
    Node::spawn(1, &mut states);
    let edges = vec![Edge{head: &states[0], tail: &states[0], epsilon: false, label: String::from("a")}];
    assert!(Graph::make(&states, &edges).edges.len() == 1);
}

#[test]
fn spawn_nodes_test() {
    let mut nodes = vec![];
    Node::spawn(1, &mut nodes);
    assert!(nodes.len() == 1);
}
#[test]
fn node_equality() {
    // eq
    assert_eq!(Node{v: 0}, Node{v: 0});
    // not eq
    assert_ne!(Node{v: 0}, Node{v: 1});
}

#[test]
fn edge_is_loop() {
    let n = Node {v: 0};
    let edge = Edge{ head: &n, tail: &n, epsilon: false, label: String::from("loop") };
    assert!(edge.is_loop());
}

#[test]
fn edge_is_not_loop() {
    let n = Node {v: 0};
    let n2 = Node {v: 1};
    let edge = Edge{ head: &n, tail: &n2, epsilon: false, label: String::from("no loop") };
    assert!(!edge.is_loop());
}
#[test]
fn spawn_loop_test() {
    let node = Node { v: 0 };
    let mut edges = vec![];
    Edge::spawn_loop(&node, "hoge", &mut edges);
    assert!(edges.len() == 1);
}
#[test]
fn edge_spawn_test() {
    let node1 = Node { v: 0 };
    let node2 = Node { v: 1 };
    let mut edges = vec![];
    Edge::spawn(&node1, &node2, "fuga", &mut edges);
    assert!(edges.len() == 1);
}
#[test]
fn edge_spawn_sort_test() {
    let node1 = Node { v: 1 };
    let node2 = Node { v: 0 };
    let mut edges = vec![];
    Edge::spawn(&node1, &node2, "fuga", &mut edges);
    assert!(edges[0].head.v == 0 && edges[0].tail.v == 1);
}