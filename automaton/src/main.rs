mod automaton;
use crate::automaton::*;

fn main() {
    let states = State::spawn(4);
    let mut edges: Vec<Edge> = vec![];

    // (a|b)*abb
    edges.push(Edge::spawn_loop(states[0], "a"));
    edges.push(Edge::spawn_loop(states[0], "b"));
    edges.push(Edge::spawn(states[0], states[1], "a"));
    edges.push(Edge::spawn(states[1], states[2], "b"));
    edges.push(Edge::spawn(states[2], states[3], "b"));

    let nfa = NFA::make(&states, &edges);

    println!("graph: {:#?}", nfa);
}
