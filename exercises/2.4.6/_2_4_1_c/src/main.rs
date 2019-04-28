use std::env;

fn s(look_ahead: &mut Vec<char>) {
    if look_ahead.is_empty() {
        return
    } else {
        if look_ahead[0] == '0' && look_ahead[look_ahead.len()-1] == '1' {
            println!("{:?}", look_ahead);
            let mut look_ahead: Vec<char> = 
                look_ahead.iter()
                          .enumerate()
                          .filter(|&(i, _)| i != 0 && i != look_ahead.len()-1)
                          .map(|(_, e)| *e)
                          .collect();
            s(&mut look_ahead);
        } else {
            println!("Unexpected syntax error");
            return
        }
    }
}

fn main() {
    let mut args :Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please input: S -> 0S1 | 01");
        println!("ex: 000111");
        return
    } else {
        s(&mut args[1].chars().collect());
    }
}