use clap::ArgMatches;
use crate::connection::{ConnectionRepository, Database};
use anyhow::Result;
use anyhow::Context;
use std::fs::File;

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    let source_str = args.value_of("source")
        .with_context(|| "Connection target not specified")?;
    let destination_file = args.value_of("destination_file")
        .with_context(|| "Destination file path not given")?;
    let connection_target = Database::from_str(connections, source_str).await?;
    let num_bytes_copied = {
        // TODO Async
        // let mut file = File::create(&destination_file).await?;
        let mut file = File::create(&destination_file)?;
        let mut guardian = connection_target.dump()?;

        // TODO Async
        std::io::copy(guardian.output(), &mut file)?
    };

    println!("Wrote {} bytes", num_bytes_copied);

    // TODO Link dumps to connections somehow

    // TODO Implement some kind of compression for the stored file. Perhaps
    // this can be gzip to maintain compatibility with mongorestore

    // TODO Implement encryption for the stored file. This should use a
    // randomly generated key saved to the connection. Connections can
    // optionally also be encrypted using a key derived from a user-input
    // password.

    Ok(())
}
