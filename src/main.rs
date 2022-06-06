use std::{env};

use badb::Badb;

mod badb;


fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let mut badb = Badb::new();
    match badb.exec(&args) {
        Ok(result) => print!("{}", result),
        Err(err) => eprint!("{}", err)
    };
}