extern crate chrono;

use self::chrono::*;
use hyper::uri::RequestUri;
use nickel::{Request, Response, MiddlewareResult, Continue, Middleware};

pub enum LogLevel {
    Debug = 5,
    Info = 4,
    Warn = 3,
    Error = 2
}

pub struct Logger<'a> {
    pub name: &'a str,
    pub level: i32
}

impl <'a> Logger<'a> {
    pub fn new(name: &'a str) -> Logger {
        Logger { name: name, level: LogLevel::Debug as i32 }
    }

    pub fn debug(&self, msg: String) {
        if self.level >=  LogLevel::Debug as i32 {
            let dt = Local::now();
            println!("{} - {} [Debug]:: {}", dt.format("%Y-%m-%d %H:%M:%S"), self.name, msg);
        }
    }

    pub fn info(&self, msg: String) {
        if self.level >=  LogLevel::Info as i32{
            let dt = Local::now();
            println!("{} - {} [Info]:: {}", dt.format("%Y-%m-%d %H:%M:%S"), self.name, msg);
        }
    }

    pub fn warn(&self, msg: String) {
        if self.level >=  LogLevel::Warn as i32 {
            let dt = Local::now();
            println!("{} - {} [Warn]:: {}", dt.format("%Y-%m-%d %H:%M:%S"), self.name, msg);
        }
    }

    pub fn error(&self, msg: String) {
        if self.level >=  LogLevel::Error as i32 {
            let dt = Local::now();
            println!("{} - {} [Error]:: {}", dt.format("%Y-%m-%d %H:%M:%S"), self.name, msg);
        }
    }
}

pub struct LoggingMiddleware {
    logger: Logger<'static>,
}

impl LoggingMiddleware {
    pub fn new(name: &'static str) -> LoggingMiddleware {
        LoggingMiddleware {
            logger: Logger { name: name, level: LogLevel::Info as i32 }
        }
    }
}

impl <D: 'static> Middleware<D> for LoggingMiddleware {
    fn invoke<'mw, 'conn>(&'mw self, request: &mut Request<'mw, 'conn, D>, response: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        let address = match request.origin.uri {
            RequestUri::AbsolutePath(ref path) => { path.to_owned() },
            RequestUri::AbsoluteUri(ref uri) => {
                match uri.fragment {
                    Some(ref fragment) => { fragment.to_owned() }
                    None => { "/".to_owned() }
                }
            },
            RequestUri::Authority(ref authority) => { authority.to_owned() },
            RequestUri::Star => { "*".to_owned() },
        };
        self.logger.info(format!("{} - {} {:?}", request.origin.remote_addr, request.origin.method, address));
        Ok(Continue(response))
    }
}
