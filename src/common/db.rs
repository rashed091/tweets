use diesel::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use once_cell::sync::OnceCell;

pub type DBPool = Pool<ConnectionManager<PgConnection>>;
pub type DBPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;
pub type Connection = DBPooledConnection;

/// wrapper function for a database pool
#[derive(Clone)]
pub struct Database {
    pub pool: &'static DBPool,
}

impl Default for Database {
    fn default() -> Self {
        Self::new()
    }
}

impl Database {
    pub fn new() -> Database {
        Database {
            pool: Self::get_or_init_pool(),
        }
    }

    pub fn get_connection(&self) -> Connection {
        self.pool.get().unwrap()
    }

    fn get_or_init_pool() -> &'static DBPool {
        static POOL: OnceCell<DBPool> = OnceCell::new();
				let manager = ConnectionManager::<PgConnection>::new(Self::connection_url());
				let pool = Pool::builder()
						.connection_timeout(std::time::Duration::from_secs(5))
						.build(manager).unwrap();
						// .expect("Failed to create pool");

        POOL.get_or_init(|| {pool})
    }

    pub fn connection_url() -> String {
        std::env::var("DATABASE_URL").expect("DATABASE_URL environment variable expected.")
    }
}
