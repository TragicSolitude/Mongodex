use crate::RUNTIME;
use super::ConnectionRepository;
use super::ServerList;
use super::Server;

// TODO Somehow add more descriptive error messages to these. Not sure if
// changing the error type from these methods is the correct way to go about
// that or not.

#[derive(shaku::Provider)]
#[shaku(interface = ConnectionRepository)]
pub struct SqliteConnectionRepository {
    #[shaku(provide)]
    db: Box<sqlx::SqliteConnection>
}

impl ConnectionRepository for SqliteConnectionRepository {
    fn list_connections(&mut self) -> Result<ServerList, sqlx::Error> {
        // TODO Create ServerEntry struct to represent what's actually in the
        // database

        let query = sqlx::query_as!(Server, "
            SELECT name, read_only, host, username, password, use_ssl,
                   repl_set_name, auth_source
            FROM connections");

        RUNTIME.block_on(query.fetch_all(self.db.as_mut()))
            .map(|value| value.into())
    }

    fn get_connection(&mut self, name: &str) -> Result<Server, sqlx::Error> {
        let query = sqlx::query_as!(Server, "
            SELECT name, read_only, host, username, password, use_ssl,
                   repl_set_name, auth_source
            FROM connections
            WHERE name = ?", name);

        RUNTIME.block_on(query.fetch_one(self.db.as_mut()))
    }

    fn add_connection(&mut self, info: &Server) -> Result<(), sqlx::Error> {
        let query = sqlx::query!("
            INSERT INTO connections (name, read_only, host, username, password,
                                     use_ssl, repl_set_name, auth_source)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
            info.name, info.read_only, info.host, info.username, info.password,
            info.use_ssl, info.repl_set_name, info.auth_source);

        RUNTIME.block_on(query.execute(self.db.as_mut()))
            .map(|_result| ())
    }

    fn replace_connection(&mut self, info: &Server) -> Result<(), sqlx::Error> {
        let query = sqlx::query!("
            UPDATE connections
            SET read_only = ?, host = ?, username = ?, password = ?,
                use_ssl = ?, repl_set_name = ?, auth_source = ?
            WHERE name = ?",
            info.read_only, info.host, info.username, info.password,
            info.use_ssl, info.repl_set_name, info.auth_source, info.name);

        RUNTIME.block_on(query.execute(self.db.as_mut())).map(|_result| ())
    }

    fn remove_connection(&mut self, name: &str) -> Result<(), sqlx::Error> {
        let query = sqlx::query!("
            DELETE FROM connections WHERE name = ?", name);

        RUNTIME.block_on(query.execute(self.db.as_mut())).map(|_result| ())
    }
}
