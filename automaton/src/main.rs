mod automaton;
use crate::automaton::*;

fn main() {
    let mut states: Vec<Node> = vec![];
    let mut edges: Vec<Edge> = vec![];
    State::spawn(4, &mut states);

    // (a|b)*abb
    Edge::spawn_loop(&states[0], "a", &mut edges);
    Edge::spawn_loop(&states[0], "b", &mut edges);
    Edge::spawn(&states[0], &states[1], "a", &mut edges);
    Edge::spawn(&states[1], &states[2], "b", &mut edges);
    Edge::spawn(&states[2], &states[3], "b", &mut edges);

    let nfa = NFA::make(&states, &edges);

    println!("graph: {:#?}", nfa);
}
