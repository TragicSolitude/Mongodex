use clap::Clap;
use crate::error::Error;
use crate::cli::prompt_db;
use crate::connection_info::ConnectionInfo;

#[derive(Clap)]
pub struct RestoreCommand {
    /// Which saved connection to use
    #[clap()]
    connection_name: String,
    /// When connecting to a cluster with multiple databases, specifies the
    /// database to restore the dump to
    #[clap(short = "d", long = "db")]
    db: Option<String>
}

impl RestoreCommand {
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

        // let num_bytes_copied = {
            let mut file = std::fs::File::open("./dbdump.bin")?;
            let mut guardian = connection_info.restore(&db, None)?;
            
            let num_bytes_copied = std::io::copy(&mut file, guardian.input())?;
        // };

        println!("Wrote {} bytes", num_bytes_copied);

        Ok(())
    }
}