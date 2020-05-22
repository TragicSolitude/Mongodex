use std::path::PathBuf;
use clap::Clap;
use crate::error::Error;
use crate::connection::Database;

#[derive(Clap)]
pub struct RestoreCommand {
    /// File to restore, created by the dump subcommand.
    #[clap()]
    dump_file: PathBuf,

    /// The database to restore to
    #[clap()]
    destination: Database,

    // TODO Replace this by tracking backup files in embedded db
    /// If renaming the database specify the original name
    #[clap(short = "f", long = "from")]
    from: Option<String>
}

impl RestoreCommand {
    pub fn handle(&self) -> Result<(), Error> {
        let num_bytes_copied = {
            let mut file = std::fs::File::open(&self.dump_file)?;
            let mut guardian = self.destination.restore(self.from.as_deref())?;
            
            std::io::copy(&mut file, guardian.input())?
        };

        println!("Wrote {} bytes", num_bytes_copied);

        Ok(())
    }
}