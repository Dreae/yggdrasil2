extern crate chrono;

use self::chrono::*;

pub enum LogLevel {
    Debug = 5,
    Info = 4,
    Warn = 3,
    Error = 2
}

pub struct Logger<'l> {
    name: &'l str,
    level: i32
}

impl<'l> Logger<'l> {
    pub fn new(name: &'l str) -> Logger<'l> {
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
