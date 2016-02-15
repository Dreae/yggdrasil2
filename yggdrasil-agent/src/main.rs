extern crate getopts;
extern crate hyper;

#[cfg(windows)] extern crate kernel32;
#[cfg(windows)] extern crate winapi;

use std::env;
use std::io::Read;

use getopts::Options;

mod init;
mod steamcmd;
mod procapi;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    let mut opts = Options::new();
    opts.reqopt("r", "register-url", "Master server registration URL", "Master server URL");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => { m }
        Err(f) => { panic!(f.to_string()) }
    };

    init::run(matches.opt_str("r").unwrap()).unwrap();
}
