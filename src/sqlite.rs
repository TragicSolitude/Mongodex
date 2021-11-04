use std::error::Error;
use crate::RUNTIME;
use crate::DATA_DIRECTORY;

// TODO more docs
/// Provider for a SQLite database connection
pub struct SqliteConnection;

impl<M: shaku::Module> shaku::Provider<M> for SqliteConnection {
    type Interface = sqlx::SqliteConnection;

    fn provide(_module: &M) -> Result<Box<sqlx::SqliteConnection>, Box<dyn Error + 'static>> {
        use sqlx::ConnectOptions;

        let db_path = DATA_DIRECTORY.join("connections.db");
        RUNTIME.block_on(async {
            let mut connection = sqlx::sqlite::SqliteConnectOptions::new()
                .filename(db_path)
                .create_if_missing(true)
                .connect().await
                .expect("Could not connect to the config db");

            // Run migrations against database to ensure schema is valid
            sqlx::migrate!("./migrations")
                .run(&mut connection).await
                .expect("Failed to update config db");

            Ok(Box::new(connection))
        })
    }
}
