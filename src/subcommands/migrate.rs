use clap::Clap;
use crate::error::Error;
use crate::cli::prompt_db;
use crate::connection_info::ConnectionInfo;

// TODO Support directly specifying database

#[derive(Clap)]
pub struct MigrateCommand {
    /// Source connection
    #[clap()]
    source_connection: String,
    /// Destination connection
    #[clap(long = "--to")]
    destination_connection: String
}

impl MigrateCommand {
    pub fn handle(&self) -> Result<(), Error> {
        let source = ConnectionInfo::load_saved(&self.source_connection)?;
        let databases = source.list_databases()?;
        let source_db_name = prompt_db(&databases)?;

        let destination = ConnectionInfo::load_saved(&self.destination_connection)?;
        let dest_db_list = destination.list_databases()?;
        let dest_db_name = prompt_db(&dest_db_list)?;

        let num_bytes_copied = {
            let mut dest_guardian = destination.restore(source_db_name, Some(dest_db_name))?;
            let mut source_guardian = source.dump(source_db_name)?;

            std::io::copy(source_guardian.output(), dest_guardian.input())?
        };
        println!("Migrated {} bytes", num_bytes_copied);

        Ok(())
    }
}