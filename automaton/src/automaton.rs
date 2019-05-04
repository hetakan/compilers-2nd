pub type State = Node;
pub type NFA<'a> = Graph<'a>;

#[derive(Debug, Copy, Clone)]
pub struct Node {
    v: i32
}
impl Node {
    pub fn spawn(n: i32) -> Vec<Self> {
        (0..n).fold(vec![], |mut acc, i| { acc.push(Node{v: i}); acc })
    }
}
impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.v == other.v
    }
}

#[derive(Debug)]
pub struct Edge {
    head: Node,
    tail: Node,
    label: String,
    epsilon: bool,
}
impl Edge {
    fn spawn_inner(s1: State, s2: State, name: &str, epsilon: bool) -> Self {
        Self {
            head: s1,
            tail: s2,
            label: String::from(name),
            epsilon: epsilon,
        }
    }
    pub fn spawn(s1: State, s2: State, name: &str) -> Self {
        if s1.v > s2.v {
            Self::spawn_inner(s2, s1, name, false)
        } else {
            Self::spawn_inner(s1, s2, name, false)
        }
    }
    pub fn spawn_loop(s: State, name: &str) -> Self {
        Self::spawn_inner(s, s, name, false)
    } 
    pub fn spawn_epsilon(s1: State, s2: State) -> Self {
        Self::spawn_inner(s1, s2, "epsilon", true)
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
    states: &'a Vec<State>,
    edges: &'a Vec<Edge>,
}
impl<'a> Graph<'a> {
    pub fn make(states: &'a Vec<State>, edges: &'a Vec<Edge>) -> Graph<'a> {
        Self { states: states, edges: edges }
    }
    pub fn move_states(&self, states: &Vec<State>, a: &str) -> Vec<State> {
        states.iter().fold(vec![], |mut acc, &s| {
            self.edges
                .iter()
                .filter(|&e| e.label == a && e.head == s)
                .map(|e| e.tail)
                .for_each(|node| acc.push(node));
            acc
        })
    }
    pub fn epsilon_nearest_closure(&self, s: State) -> Vec<State> {
        self.edges
            .iter()
            .filter(|&edge| edge.is_epsilon() && s == edge.head)
            .map(|edge| edge.tail)
            .collect()
    }
    //pub fn epsilon_closure(&self, states: &Vec<State>) -> Vec<State> {
    //    let mut stack = states;
    //    let mut closure = states;
    //    while !stack.is_empty() {
    //        let t = stack.pop();
    //    }
    //}
}
#[test]
fn move_states_test() {
    let states = Node::spawn(5);
    let edges = vec![
        Edge{head: states[0], tail: states[1], epsilon:false, label: "a".to_string()},
        Edge{head: states[1], tail: states[2], epsilon:false, label: "a".to_string()},
        Edge{head: states[1], tail: states[4], epsilon:false, label: "a".to_string()},
        Edge{head: states[2], tail: states[3], epsilon:false, label: "b".to_string()},
        Edge{head: states[3], tail: states[4], epsilon:false, label: "a".to_string()},
    ];
    let g = Graph::make(&states, &edges);
    let ret = g.move_states(&vec![states[1]], "a");
    assert_eq!(ret.len(), 2);
    assert!(ret.contains(&states[2]));
    assert!(ret.contains(&states[4]));
}
//#[test]
//fn epsilon_closure_test() {
//    // NFA of (a|b)*abb (compilers-2nd P.166, fig. 3.34)
//    let states = Node::spawn(11);
//    let edges = vec![
//        Edge{head: states[0], tail: states[1], epsilon: true, label: "epsilon"},
//        Edge{head: states[0], tail: states[7], epsilon: true, label: "epsilon"},
//        Edge{head: states[1], tail: states[2], epsilon: true, label: "epsilon"},
//        Edge{head: states[2], tail: states[3], epsilon: false, label: "a"},
//        Edge{head: states[3], tail: states[6], epsilon: true, label: "epsilon"},
//        Edge{head: states[1], tail: states[4], epsilon: true, label: "epsilon"},
//        Edge{head: states[4], tail: states[5], epsilon: false, label: "b"},
//        Edge{head: states[5], tail: states[6], epsilon: true, label: "epsilon"},
//        Edge{head: states[6], tail: states[1], epsilon: true, label: "epsilon"},
//        Edge{head: states[6], tail: states[7], epsilon: true, label: "epsilon"},
//        Edge{head: states[7], tail: states[8], epsilon: false, label: "a"},
//        Edge{head: states[8], tail: states[9], epsilon: false, label: "b"},
//        Edge{head: states[9], tail: states[10], epsilon: false, label: "b"},
//    ];
//    let g = Graph::make(&states, &edges);
//    let ret = g.epsilon_closure(&vec![Node{v:0}]);
//    println!("{:#?}", ret);
//    assert_eq!(ret.len(), 5);
//}

#[test]
fn epsilon_nearest_closure_test() {
    let states = Node::spawn(2);
    let edges = vec![Edge{head: states[0], tail: states[1], epsilon: true, label: String::from("epsilon")}];
    let g = Graph::make(&states, &edges);
    let iter = g.epsilon_nearest_closure(Node{v: 0});
    assert_eq!(iter[0], Node{v: 1});
}

#[test]
fn epsilon_nearest_closure_nothing_test() {
    let states = Node::spawn(2);
    let edges = vec![Edge{head: states[0], tail: states[1], epsilon: false, label: String::from("epsilon")}];
    let g = Graph::make(&states, &edges);
    let iter = g.epsilon_nearest_closure(Node{v: 0});
    assert!(iter.is_empty());
}

#[test]
fn graph_make_test() {
    let states = Node::spawn(1);
    let edges = vec![Edge{head: states[0], tail: states[0], epsilon: false, label: String::from("a")}];
    assert!(Graph::make(&states, &edges).edges.len() == 1);
}

#[test]
fn spawn_nodes_test() {
    let n = 2;
    Node::spawn(n)
        .iter()
        .for_each(|node| assert_eq!(*node, Node{v: node.v}));
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
    let edge = Edge{ head: n, tail: n, epsilon: false, label: String::from("loop") };
    assert!(edge.is_loop());
}

#[test]
fn edge_is_not_loop() {
    let n = Node {v: 0};
    let n2 = Node {v: 1};
    let edge = Edge{ head: n, tail: n2, epsilon: false, label: String::from("no loop") };
    assert!(!edge.is_loop());
}
#[test]
fn spawn_loop_test() {
    let node = Node { v: 0 };
    let edge = Edge::spawn_loop(node, "hoge");

    assert!(edge.head == Node{v:0} && edge.tail == Node{v:0});
}
#[test]
fn edge_spawn_test() {
    let n1 = Node { v: 0 };
    let n2 = Node { v: 1 };
    let edge = Edge::spawn(n1, n2, "fuga");
    assert!(edge.head == n1 && edge.tail == n2);
}
#[test]
fn edge_spawn_sort_test() {
    let n1 = Node { v: 1 };
    let n2 = Node { v: 0 };
    let edge = Edge::spawn(n1, n2, "fuga");
    assert!(edge.head == n2 && edge.tail == n1);
}