use clap::Clap;
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;
use crate::error::Error;
use crate::connection::Database;

#[derive(Clap)]
pub struct DumpCommand {
    /// Which saved connection to use. This should be specified in the format
    /// [database@]saved-connection where a specific database can be provided. On
    /// connections with the database name saved, the database provided here is ignored.
    /// If not saved or provided here, the database name will be prompted from a list of
    /// databases currently on the server.
    #[clap()]
    connection_target: String,

    /// The destination file path for the dump. If the file doesn't exist it will be created
    /// otherwise it is truncated before dumping the databse.
    #[clap()]
    destination_file: PathBuf
}

impl DumpCommand {
    pub fn handle(&self) -> Result<(), Error> {
        // For some reason clap parses the field twice which causes 2 db prompts for the
        // user
        let connection_target = Database::from_str(&self.connection_target)?;
        let num_bytes_copied = {
            let mut file = File::create(&self.destination_file)?;
            let mut guardian = connection_target.dump()?;
    
            std::io::copy(guardian.output(), &mut file)?
        };
        println!("Wrote {} bytes", num_bytes_copied);

        // TODO Link dumps to connections

        // TODO Implement some kind of compression for the stored file. Perhaps
        // this can be gzip to maintain compatibility with mongorestore

        // TODO Implement encryption for the stored file. This should use a
        // randomly generated key saved to the connection. Connections can
        // optionally also be encrypted.
    
        Ok(())
    }
}