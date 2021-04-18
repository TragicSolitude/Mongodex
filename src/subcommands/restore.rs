use clap::ArgMatches;
use crate::connection::{ConnectionRepository, Database};
use anyhow::Result;
use anyhow::Context;
use std::fs::File;

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    let dump_file_path = args.value_of("dump_file")
        .with_context(|| "Dump file path was not given")?;
    let destination_name = args.value_of("destination")
        .with_context(|| "Destination was not specified")?;
    let from = args.value_of("from");

    let destination = Database::from_str(connections, destination_name).await?;

    if destination.read_only {
        return Err(anyhow!("Destination is read only"));
    }

    let num_bytes_copied = {
        // TODO Async
        // let mut file = File::create(&destination_file).await?;
        let mut file = File::open(dump_file_path)?;
        let mut guardian = destination.restore(from.as_deref())?;

        // TODO Async
        std::io::copy(&mut file, guardian.input())?
    };

    println!("Wrote {} bytes", num_bytes_copied);

    Ok(())
}
