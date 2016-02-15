extern crate frank_jwt;
extern crate cookie;
extern crate hyper;

use self::frank_jwt::{decode, Algorithm};
use nickel::{Request, Response, MiddlewareResult};
use nickel::status::StatusCode;
use self::cookie::Cookie;
use self::hyper::header;

pub fn authentication_middleware<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    if !req.origin.headers.has::<header::Cookie>() {
        return res.error(StatusCode::Unauthorized, "Unauthorized");
    }

    let cookie_jar = req.origin.headers.get::<header::Cookie>().unwrap().to_cookie_jar(b"abcd");
    match cookie_jar.find("yggdrasil_token") {
        Some(cookie) => {
            match decode(cookie.value, "butts".to_owned(), Algorithm::HS512) {
                Ok(_) => { res.next_middleware() }
                Err(_) => { res.error(StatusCode::Unauthorized, "Unauthorized") }
            }
        }
        None => { res.error(StatusCode::Unauthorized, "Unauthorized") }
    }
}
