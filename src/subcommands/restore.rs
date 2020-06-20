use std::str::FromStr;
use std::path::PathBuf;
use clap::Clap;
use crate::error::Error;
use crate::connection::Database;

#[derive(Clap)]
pub struct RestoreOptions {
    /// File to restore, created by the dump subcommand.
    #[clap()]
    dump_file: PathBuf,

    /// The database to restore to
    #[clap()]
    destination: String,

    // TODO Replace this by tracking backup files in embedded db
    /// If renaming the database specify the original name
    #[clap(short = "f", long = "from")]
    from: Option<String>
}

pub fn run(options: &RestoreOptions) -> Result<(), Error> {
    // For some reason clap parses the field twice which causes 2 db prompts for the
    // user
    let destination = Database::from_str(&options.destination)?;

    if destination.read_only {
        return Err(Error::WriteToReadOnlyConnection);
    }
    
    let num_bytes_copied = {
        let mut file = std::fs::File::open(&options.dump_file)?;
        let mut guardian = destination.restore(options.from.as_deref())?;
        
        std::io::copy(&mut file, guardian.input())?
    };

    println!("Wrote {} bytes", num_bytes_copied);

    Ok(())
}