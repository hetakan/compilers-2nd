mod automaton;
use crate::automaton::*;

fn main() {
    let states = State::spawn(4);
    // (a|b)*abb
    let edges = vec![
        Edge::spawn_loop(states[0], "a"),
        Edge::spawn_loop(states[0], "b"),
        Edge::spawn(states[0], states[1], "a"),
        Edge::spawn(states[1], states[2], "b"),
        Edge::spawn(states[2], states[3], "b"),
    ];
    let nfa = NFA::make(&states, &edges);

    println!("graph: {:#?}", nfa);
}
