use diesel::pg::PgConnection;
use r2d2::{Config, InitializationError, Pool};
use r2d2_diesel::ConnectionManager;

pub type ConnectionPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect<U>(url: U) -> Result<ConnectionPool, InitializationError>
where
    U: Into<String>,
{
    let cfg = Config::default();
    let mgr = ConnectionManager::<PgConnection>::new(url.into());

    Pool::new(cfg, mgr)
}
