#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Destination is marked read-only")]
    WriteToReadOnlyConnection,
    #[error("No connection specified")]
    NoConnection,
    #[error("No such connection '{0}'")]
    NoSuchConnection(String),
    #[error("DB Error '{0}'")]
    Sled(#[from] sled::Error),
    #[error("Serialization Error '{0}'")]
    Bincode(#[from] bincode::Error),
    #[error("IO Error '{0}'")]
    Io(#[from] std::io::Error),
    #[error("Invalid UTF8 Encountered")]
    UTF8(#[from] std::str::Utf8Error),
    #[error("Mongo Error '{0}'")]
    Mongo(#[from] mongodb::error::Error)
}