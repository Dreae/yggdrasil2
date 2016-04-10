use nickel::{Router, HttpRouter};

mod authentication;

pub mod internal;

pub fn init_router() -> Router {
    let mut api_router = Router::new();

    api_router.post("/api/login", authentication::login);

    api_router.get("/api/restricted/**", authentication::authentication_middleware);
    api_router.post("/api/restricted/**", authentication::authentication_middleware);
    api_router.put("/api/restricted/**", authentication::authentication_middleware);
    api_router.delete("/api/restricted/**", authentication::authentication_middleware);
    // TODO: next_middleware() seems to be handing requests back to the server
    // meaning this handler never gets called - investigate.
    api_router.get("/api/restricted/session", middleware! { |_, res|
        "1"
    });

    return api_router;
}
