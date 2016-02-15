#[macro_use] extern crate nickel;
extern crate toml;
extern crate mysql;

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::Read;
use std::fs::File;
use std::process;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, StaticFilesHandler};
use mysql::conn::MyOpts;
use mysql::conn::pool::MyPool;

mod api;
mod logging;

fn hello_world<'mw>(_: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let data: HashMap<String, String> = HashMap::new();
    res.render("static/index.html", &data)
}

fn main() {
    let config_path = Path::new("config.toml");
    if !fs::metadata(config_path).is_ok() {
        //First run
        process::exit(1);
    }
    let logger = logging::Logger::new("server");

    let mut config_file = File::open(config_path).unwrap();
    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => { }
        Err(e) => { panic!("Error reading config file! {}", e) }
    };

    let ref db_value = toml::Parser::new(&config_string).parse().unwrap()["database"];
    let opts = MyOpts {
        user: db_value.lookup("user").map(|usr| usr.as_str().map(|s| s.to_string()).unwrap()),
        pass:  db_value.lookup("password").map(|pas| pas.as_str().map(|s| s.to_string()).unwrap()),
        tcp_addr:  db_value.lookup("host").map(|adr| adr.as_str().map(|s| s.to_string()).unwrap()),
        tcp_port:  db_value.lookup("port").map(|prt| prt.as_integer().unwrap()).unwrap() as u16,
        db_name:  db_value.lookup("db_name").map(|db| db.as_str().map(|s| s.to_string()).unwrap()),
        ..Default::default()
    };
    let pool = MyPool::new(opts).unwrap();

    let mut server = Nickel::new();

    server.utilize(middleware! { |request|
        logger.info(format!("{} - {} {:?}", request.origin.remote_addr, request.origin.method, request.origin.uri));
    });
    server.utilize(StaticFilesHandler::new("static/"));

    server.utilize(api::init_router(pool));
    server.get("**", hello_world);

    server.listen("127.0.0.1:6767");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert() {
        assert!(true);
    }
}
