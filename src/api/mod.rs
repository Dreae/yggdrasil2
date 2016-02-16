use nickel::{Router, HttpRouter};

mod authentication;

pub fn init_router() -> Router {
    let mut api_router = Router::new();
    api_router.get("/api/restricted/**", authentication::authentication_middleware);
    api_router.post("/api/restricted/**", authentication::authentication_middleware);
    api_router.post("/api/login", authentication::login);
    api_router.get("/api/restricted/session", middleware! { |_, res|
        "1"
    });

    return api_router;
}
