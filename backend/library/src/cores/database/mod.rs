use anyhow::Result;
use diesel;
use diesel::pg::PgConnection;
use diesel::r2d2::{ Pool, PooledConnection, ConnectionManager };

pub type PgPool = Pool<ConnectionManager<PgConnection>>;
pub type PgPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

#[derive(Clone, Debug)]
pub struct DBManager {
    pub reader: PgPool,
    pub writer: PgPool
}

impl DBManager {
    pub fn init() -> Result<Self> {
        // Set writer url
        let writer = std::env::var("DATABASE_WRITE_URL")?;

        // Set reader url
        let reader  = std::env::var("DATABASE_READ_URL")
            .unwrap_or(writer.clone());

        // Check if any of the database connection is empty
        if writer.is_empty() || reader.is_empty() {
            return Err(anyhow::anyhow!("Invalid database url configuration"));
        }

        Ok(DBManager {
            writer: Pool::builder()
                .build(ConnectionManager::<PgConnection>::new(writer))
                .map_err(|e| anyhow::anyhow!(e))?,
            reader: Pool::builder()
                .build(ConnectionManager::<PgConnection>::new(reader))
                .map_err(|e| anyhow::anyhow!(e))?,
        })
    }

    pub fn reader(&self) -> PgPooledConnection {
        self.reader.get()
            .expect("Unable to initialize database reader...")
    }

    pub fn writer(&self) -> PgPooledConnection {
        self.writer.get()
            .expect("Unable to initialize database writer...")
    }
}