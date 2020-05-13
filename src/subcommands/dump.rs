use clap::Clap;
use crate::error::Error;
use crate::cli::prompt_db;
use crate::connection_info::ConnectionInfo;

#[derive(Clap)]
pub struct DumpCommand {
    /// Which saved connection to use
    #[clap()]
    connection_name: String,
    /// When connecting to a cluster with multiple databases, specifies the
    /// specific database to dump
    #[clap(short = "d", long = "db")]
    db: Option<String>
}

impl DumpCommand {
    pub fn handle(&self) -> Result<(), Error> {
        let connection_info = ConnectionInfo::load_saved(&self.connection_name)?;

        let items;
        let db = match &self.db {
            Some(specified_db) => specified_db,
            None => {
                items = connection_info.list_databases()?;
                prompt_db(&items)?
            }
        };

        let num_bytes_copied = {
            let mut file = std::fs::File::create("./dbdump.bin")?;
            let mut guardian = connection_info.dump(db)?;
    
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