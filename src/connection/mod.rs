mod database;
mod server;
mod repository;

// pub use database::Database;
pub use server::Server;
pub use server::ServerList;
pub use database::Database;

// TODO Move this into submodule
pub use repository::SqliteConnectionRepository;

pub trait ConnectionRepository {
    fn list_connections(&mut self) -> Result<ServerList, sqlx::Error>;
    fn get_connection(&mut self, name: &str) -> Result<Server, sqlx::Error>;
    fn add_connection(&mut self, info: &Server) -> Result<(), sqlx::Error>;
    fn replace_connection(&mut self, info: &Server) -> Result<(), sqlx::Error>;
    fn remove_connection(&mut self, name: &str) -> Result<(), sqlx::Error>;
}
