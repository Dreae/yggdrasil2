#[macro_use] extern crate nickel;

use std::collections::HashMap;

use nickel::{Nickel, Request, Response, HttpRouter, MiddlewareResult, StaticFilesHandler};

fn hello_world<'mw>(_: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let data: HashMap<String, String> = HashMap::new();
    res.render("static/index.tpl", &data)
}

fn main() {
    let mut server = Nickel::new();

    server.utilize(middleware! { |request|
        println!("Logging Request: {:?}", request.origin.uri);
    });
    server.get("/", hello_world);
    server.get("/static", StaticFilesHandler::new("static/"));

    server.listen("127.0.0.1:6767");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_assert() {
        assert!(true);
    }
}
