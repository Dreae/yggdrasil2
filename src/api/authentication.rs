extern crate frank_jwt;
extern crate hyper;

use nickel::{Request, Response, MiddlewareResult, JsonBody};
use nickel::status::StatusCode;
use cookie::{Cookie, CookieJar};
use self::hyper::header;
use mysql::conn::Row;
use mysql::error::Error;
use mysql;

use middleware::MysqlRequestExtensions;
use password;

pub fn authentication_middleware<'mw>(req: &mut Request, res: Response<'mw>) -> MiddlewareResult<'mw> {
    if !req.origin.headers.has::<header::Cookie>() {
        return res.error(StatusCode::Unauthorized, "Unauthorized");
    }

    let cookie_jar = req.origin.headers.get::<header::Cookie>().unwrap().to_cookie_jar(&[]);
    match cookie_jar.find("yggdrasil_token") {
        Some(cookie) => {
            // TODO: Get the secret key to this middleware (Request extension for server settings?)
            match frank_jwt::decode(cookie.value, "secret123".to_owned(), frank_jwt::Algorithm::HS512) {
                Ok(_) => { res.next_middleware() }
                Err(_) => { res.error(StatusCode::Unauthorized, "Unauthorized") }
            }
        }
        None => { res.error(StatusCode::Unauthorized, "Unauthorized") }
    }
}

#[derive(RustcDecodable, RustcEncodable)]
struct LoginRequest {
    username: String,
    password: String
}

pub fn login<'mw>(req: &mut Request, mut res: Response<'mw>) -> MiddlewareResult<'mw> {
    let body = req.json_as::<LoginRequest>();
    if !body.is_ok() {
        return res.error(StatusCode::BadRequest, "Bad Request");
    }
    let form_data = body.unwrap();

    let pool = req.db_connection();
    let mut stmt = pool.prepare("SELECT * FROM users WHERE username = ?").unwrap();

    match stmt.execute((&form_data.username,)) {
        Ok(result) => {
            let mut rows: Vec<Result<Row, Error>> = result.collect();
            return match rows.pop() {
                Some(row) => {
                    let (username, password): (String, String) = mysql::from_row(row.unwrap());
                    if password::verify_password(form_data.password, password) {
                        let jar: CookieJar = if !req.origin.headers.has::<header::Cookie>() {
                            CookieJar::new(&[])
                        } else {
                            req.origin.headers.get::<header::Cookie>().unwrap().to_cookie_jar(&[])
                        };

                        let mut payload = frank_jwt::Payload::new();
                        payload.insert("username".to_owned(), username);
                        let header = frank_jwt::Header::new(frank_jwt::Algorithm::HS512);

                        jar.add(Cookie::new("yggdrasil_token".to_owned(), frank_jwt::encode(header, "secret123".to_owned(), payload.clone())));
                        res.headers_mut().set::<header::SetCookie>(header::SetCookie::from_cookie_jar(&jar));

                        res.send("1")
                    } else {
                        res.error(StatusCode::Unauthorized, "Unauthorized")
                    }
                }
                None => { res.error(StatusCode::Unauthorized, "Unauthorized") }
            };
        }
        Err(_) => { return res.error(StatusCode::Unauthorized, "Unauthorized"); }
    };
}
