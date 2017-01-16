#![feature(plugin)]
#![plugin(rocket_codegen)]
#![feature(drop_types_in_const)]
#![feature(const_fn)]

#[macro_use] extern crate diesel;
#[macro_use] extern crate diesel_codegen;
#[macro_use] extern crate lazy_static;
#[macro_use] extern crate serde_derive;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;

use diesel::pg::PgConnection;
use r2d2_diesel::ConnectionManager;
use dotenv::dotenv;
use std::env;

lazy_static! {
  pub static ref POOL: r2d2::Pool<ConnectionManager<PgConnection>> = {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL to be set");
    let config = r2d2::Config::default();
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    r2d2::Pool::new(config, manager).expect("Failed to create connection pool")
  };
}

mod middleware;

fn main() {
  dotenv().ok();

  {
    let conn = middleware::DBConnection::new();
    diesel::migrations::run_pending_migrations(&*conn).expect("Error running migrations");
  }

  rocket::ignite().mount("/api", routes![]).launch();
}