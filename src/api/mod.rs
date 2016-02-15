use nickel::{Router, HttpRouter};
use mysql::conn::pool::MyPool;

mod authentication;

pub fn init_router(pool: MyPool) -> Router {
    let mut api_router = Router::new();
    api_router.get("/api/restricted/**", authentication::authentication_middleware);
    api_router.get("/api/restricted/session", middleware! { |_, res|
        ""
    });

    return api_router;
}
