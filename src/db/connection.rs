embed_migrations!();

use std::env;

use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager};
use once_cell::sync::Lazy;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type DbConnection = r2d2::PooledConnection<ConnectionManager<PgConnection>>;

static DBPOOL: Lazy<DbPool> = Lazy::new(|| {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let cpus = num_cpus::get() as u32;
    info!("Create {} DB connections", cpus * 4);
    let pool = r2d2::Pool::builder()
        .max_size(cpus * 4)
        .build(manager)
        .expect("Failed to create DB connection pool.");
    pool
});

pub fn init() {
    let conn = connect();
    info!("Run embedded migrations");
    embedded_migrations::run_with_output(&conn, &mut std::io::stdout())
        .expect("Failed to run database migrations.");
}

pub fn connect() -> DbConnection {
    return DBPOOL.get().expect("couldn't get db connection from pool.");
}
