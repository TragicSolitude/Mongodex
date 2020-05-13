#[derive(thiserror::Error, Debug)]
pub enum Error<'a> {
    #[error("No such connection \"{0}\"")]
    NoSuchConnection(&'a str),
    #[error("DB Error \"{0}\"")]
    Sled(#[from] sled::Error),
    #[error("Serialization Error \"{0}\"")]
    Bincode(#[from] bincode::Error),
    #[error("IO Error \"{0}\"")]
    Io(#[from] std::io::Error),
    #[error("Invalid UTF8 Encountered")]
    UTF8(#[from] std::str::Utf8Error)
}