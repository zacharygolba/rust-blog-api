use std::env;

use dotenv::dotenv;
use diesel::pg::PgConnection;
use r2d2::{Config, Pool};
use r2d2_diesel::ConnectionManager;

lazy_static! {
    pub static ref CONNECTIONS: Pool<ConnectionManager<PgConnection>> = {
        dotenv().ok();

        let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let config = Config::default();
        let manager = ConnectionManager::<PgConnection>::new(url);

        Pool::new(config, manager).expect("Failed to create pool.")
    };
}
