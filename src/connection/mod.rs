mod database;
mod server;

// pub use database::Database;
pub use server::Server;
pub use server::ServerList;
pub use database::Database;
use sqlx::SqliteConnection;

// TODO Somehow add more descriptive error messages to these. Not sure if
// changing the error type from these methods is the correct way to go about
// that or not.

pub struct ConnectionRepository {
    db: SqliteConnection
}

impl ConnectionRepository {
    pub fn new(db: SqliteConnection) -> Self {
        ConnectionRepository { db }
    }

    pub async fn list_connections(&mut self) -> Result<ServerList, sqlx::Error> {
        // TODO Create ServerEntry struct to represent what's actually in the
        // database
        sqlx::query_as!(Server, "
                        SELECT name, read_only, host, username, password,
                               use_ssl, repl_set_name, auth_source
                        FROM connections")
            .fetch_all(&mut self.db).await
            .map(|value| value.into())
    }

    pub async fn get_connection(&mut self, name: &str) -> Result<Server, sqlx::Error> {
        sqlx::query_as!(Server, "
                        SELECT name, read_only, host, username, password,
                               use_ssl, repl_set_name, auth_source
                        FROM connections
                        WHERE name = ?", name)
            .fetch_one(&mut self.db).await
    }

    pub async fn add_connection(&mut self, info: &Server) -> Result<(), sqlx::Error> {
        sqlx::query!("INSERT INTO connections (name, read_only, host, username,
                                               password, use_ssl, repl_set_name,
                                               auth_source)
                     VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
                     info.name, info.read_only, info.host, info.username,
                     info.password, info.use_ssl, info.repl_set_name,
                     info.auth_source)
            .execute(&mut self.db).await
            .map(|_result| ())
    }

    pub async fn replace_connection(&mut self, info: &Server) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE connections
                     SET read_only = ?, host = ?, username = ?, password = ?,
                         use_ssl = ?, repl_set_name = ?, auth_source = ?
                     WHERE name = ?",
                     info.read_only, info.host, info.username, info.password,
                     info.use_ssl, info.repl_set_name, info.auth_source,
                     info.name)
            .execute(&mut self.db).await
            .map(|_result| ())
    }

    pub async fn remove_connection(&mut self, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM connections WHERE name = ?", name)
            .execute(&mut self.db).await
            .map(|_result| ())
    }
}
