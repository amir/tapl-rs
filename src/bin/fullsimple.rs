extern crate tapl;

use std::io;
use std::io::Write;

use tapl::tapl::fullsimple;

fn main() {
    let mut input = String::new();
    let context = tapl::tapl::fullsimple::Context::new();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("read_line error");
        input.trim();
        match fullsimple::repl(&input, context.clone()) {
            Ok(s) => println!("{}", s),
            Err(e) => println!("{:?}", e),
        }
        input.clear();
    }
}
