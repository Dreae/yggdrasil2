use std::sync::Arc;
use mysql::conn::pool::Pool;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use typemap::Key;
use plugin::{Pluggable, Extensible};

pub struct MysqlMiddleware {
    pub pool: Arc<Pool>,
}

impl MysqlMiddleware {
    pub fn new(pool: Pool) -> MysqlMiddleware {
        MysqlMiddleware {
            pool: Arc::new(pool),
        }
    }
}

impl Key for MysqlMiddleware { type Value = Arc<Pool>; }

impl <D: 'static> Middleware<D> for MysqlMiddleware {
    fn invoke<'mw, 'conn>(&'mw self, request: &mut Request<'mw, 'conn, D>, response: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        request.extensions_mut().insert::<MysqlMiddleware>(self.pool.clone());
        Ok(Continue(response))
    }
}

pub trait MysqlRequestExtensions {
    fn db_connection(&self) -> Arc<Pool>;
}

impl<'mw, 'conn> MysqlRequestExtensions for Request<'mw, 'conn> {
    fn db_connection(&self) -> Arc<Pool> {
        self.extensions().get::<MysqlMiddleware>().unwrap().clone()
    }
}
