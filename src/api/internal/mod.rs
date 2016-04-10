use nickel::Router;
use toml::Value;
use std::collections::BTreeMap;

pub fn init_router(config: &BTreeMap<String, Value>) -> Router {
    let mut router = Router::new();
    router
}
