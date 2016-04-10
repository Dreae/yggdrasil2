#[macro_use] extern crate nickel;
extern crate toml;
extern crate mysql;
extern crate plugin;
extern crate typemap;
extern crate cookie;
extern crate rustc_serialize;
extern crate hyper;

use std::collections::HashMap;
use std::path::Path;
use std::fs;
use std::io::Read;
use std::fs::File;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, StaticFilesHandler};
use mysql::Opts;
use mysql::conn::pool::Pool;

mod logging;
mod password;
mod middleware;
mod api;

fn index<'mw>(_: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    let data: HashMap<String, String> = HashMap::new();
    res.render("static/index.html", &data)
}

fn main() {
    let config_path = Path::new("config.toml");
    if !fs::metadata(config_path).is_ok() {
        // TODO: First run
        panic!("Config file not found");
    }

    let mut config_file = File::open(config_path).unwrap();
    let mut config_string = String::new();
    match config_file.read_to_string(&mut config_string) {
        Ok(_) => { }
        Err(e) => { panic!("Error reading config file! {}", e) }
    };

    let parse_result = toml::Parser::new(&config_string).parse();
    if parse_result.is_none() {
        panic!("Unable to read config file");
    }

    let config = parse_result.unwrap();
    let ref db_value = config["database"];
    let opts = Opts {
        user: db_value.lookup("user").map(|usr| usr.as_str().map(|s| s.to_string()).unwrap()),
        pass:  db_value.lookup("password").map(|pas| pas.as_str().map(|s| s.to_string()).unwrap()),
        ip_or_hostname:  db_value.lookup("host").map(|adr| adr.as_str().map(|s| s.to_string()).unwrap()),
        tcp_port:  db_value.lookup("port").map(|prt| prt.as_integer().unwrap()).unwrap() as u16,
        db_name:  db_value.lookup("db_name").map(|db| db.as_str().map(|s| s.to_string()).unwrap()),
        ..Default::default()
    };
    let pool = Pool::new(opts).unwrap();

    let mut server = Nickel::new();

    server.utilize(logging::LoggingMiddleware::new("request"));

    server.utilize(middleware::MysqlMiddleware::new(pool));
    server.utilize(StaticFilesHandler::new("static/"));

    server.utilize(api::init_router());
    server.utilize(api::internal::init_router(&config));
    server.get("**", index);

    server.listen("127.0.0.1:6767");
}
