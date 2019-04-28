use std::env;
use std::str::Chars;

fn s(look_ahead: &mut Chars) {
    match look_ahead.next() {
        Some(c) => {
            match c {
                '+' | '-' => { println!("{}", c); s(look_ahead); s(look_ahead); },
                'a'       => { println!("{}", c); return },
                _         => println!("unexpected syntax : {}", c),
            }
        },
        None => println!("Unexpected syntax error"),
    }
}

fn main() {
    let mut args :Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Please input: S -> +SS | -SS | a");
        println!("ex: +a+a-aa");
        return
    } else {
        s(&mut args[1].chars());
    }
}
