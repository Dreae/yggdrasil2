use rocket::request::{Request, FromRequest};
use rocket::outcome::Outcome;
use rocket::request;
use std::ops::Deref;
use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use r2d2::PooledConnection;
use super::POOL;

pub struct DBConnection {
  conn: PooledConnection<ConnectionManager<PgConnection>>,
}

impl DBConnection {
  pub fn new() -> DBConnection {
    DBConnection {
      conn: POOL.clone().get().unwrap(),
    }
  }
}

impl <'a, 'r> FromRequest<'a, 'r> for DBConnection {
  type Error = ();
  fn from_request(_: &'a Request<'r>) -> request::Outcome<DBConnection, ()> {
    Outcome::Success(DBConnection::new())
  }
}

impl Deref for DBConnection {
  type Target = PgConnection;

  fn deref(&self) -> &PgConnection {
    &*self.conn
  }
}