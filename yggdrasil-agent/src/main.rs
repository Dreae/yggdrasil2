extern crate getopts;
use getopts::Options;
use std::env;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut opts = Options::new();
    opts.reqopt("a", "", "API URL", "API URL");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };
    println!("Hello, world!");
}
