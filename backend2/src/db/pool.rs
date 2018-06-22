use diesel::{r2d2, sqlite::SqliteConnection};

type Connection = SqliteConnection;
type Manager = r2d2::ConnectionManager<Connection>;
pub type Pool = r2d2::Pool<Manager>;
pub type PoolConnection = r2d2::PooledConnection<Manager>;

#[derive(Debug)]
struct ConnectionCustomizer;

impl r2d2::CustomizeConnection<Connection, r2d2::Error> for ConnectionCustomizer {
    fn on_acquire(&self, conn: &mut Connection) -> ::std::result::Result<(), r2d2::Error> {
        use diesel::connection::SimpleConnection;
        conn.batch_execute("PRAGMA foreign_keys = ON;").unwrap();
        conn.batch_execute("PRAGMA journal_mode=WAL;").unwrap();
        Ok(())
    }

    fn on_release(&self, _: Connection) {}
}

pub fn build_pool(db_uri: &str) -> Result<Pool, r2d2::PoolError> {
    let manager = Manager::new(db_uri);
    let pool = Pool::builder().max_size(5).build(manager)?;
    Ok(pool)
}
