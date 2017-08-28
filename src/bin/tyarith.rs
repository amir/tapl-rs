extern crate tapl;

use std::io;
use std::io::Write;

use tapl::tapl::tyarith;

fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("read_line error");
        input.trim();
        match tyarith::run(&input) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{:?}", e),
        }
        input.clear();
    }
}
