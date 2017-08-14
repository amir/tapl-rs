extern crate tapl;

use std::io;
use std::io::Write;

use tapl::tapl::arith;

fn main() {
    let mut input = String::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("read_line error");
        input.trim();
        match arith::run(&input) {
            Ok(s) => println!("{}", s),
            _     => println!("Error"),
        }
        input.clear();
    }
}
