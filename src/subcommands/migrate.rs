use clap::Clap;
use crate::error::Error;
use crate::connection::Database;

// TODO Support directly specifying database by creating a type that implements
// From<&str> where the format 'database@connection' can be used on connections
// that have more than one database. The database is optional and if not
// provided on a connection with multiple databases it will be prompted for.

#[derive(Clap)]
pub struct MigrateCommand {
    /// Source connection
    #[clap()]
    source: Database,
    /// Destination connection
    #[clap(long = "--to")]
    destination: Database
}

impl MigrateCommand {
    pub fn handle(&self) -> Result<(), Error> {
        if self.destination.read_only {
            return Err(Error::WriteToReadOnlyConnection);
        }
        
        let num_bytes_copied = {
            let source_name = Some(self.source.db_name.as_str());
            let mut destination_guardian = self.destination.restore(source_name)?;
            let mut source_guardian = self.source.dump()?;

            std::io::copy(source_guardian.output(), destination_guardian.input())?
        };
        println!("Migrated {} bytes", num_bytes_copied);

        Ok(())
    }
}