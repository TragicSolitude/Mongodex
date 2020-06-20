use std::str::FromStr;
use clap::Clap;
use crate::error::Error;
use crate::connection::Database;

#[derive(Clap)]
pub struct MigrateOptions {
    /// Source connection
    #[clap()]
    source: String,
    /// Destination connection
    #[clap(long = "--to")]
    destination: String
}

pub fn run(options: &MigrateOptions) -> Result<(), Error> {
    // For some reason clap parses the field twice which causes 2 db prompts for the
    // user
    let source = Database::from_str(&options.source)?;
    let destination = Database::from_str(&options.destination)?;

    if destination.read_only {
        return Err(Error::WriteToReadOnlyConnection);
    }
    
    let num_bytes_copied = {
        let source_name = Some(source.db_name.as_str());
        let mut destination_guardian = destination.restore(source_name)?;
        let mut source_guardian = source.dump()?;

        std::io::copy(source_guardian.output(), destination_guardian.input())?
    };
    println!("Migrated {} bytes", num_bytes_copied);

    Ok(())
}