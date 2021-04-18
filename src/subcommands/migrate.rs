use clap::ArgMatches;
use crate::connection::{ConnectionRepository, Database};
use anyhow::Result;
use anyhow::Context;

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    let source_name = args.value_of("source")
        .with_context(|| "Source not provided")?;
    let destination_name = args.value_of("destination")
        .with_context(|| "Destination not provided")?;

    let source = Database::from_str(connections, source_name).await?;
    let destination = Database::from_str(connections, destination_name).await?;

    if destination.read_only {
        return Err(anyhow!("Destination is read only"));
    }

    let num_bytes_copied = {
        let source_db_name = Some(source.db_name.as_str());
        let mut destination_guardian = destination.restore(source_db_name)?;
        let mut source_guardian = source.dump()?;

        // TODO Async
        std::io::copy(source_guardian.output(), destination_guardian.input())?
    };
    println!("Migrated {} bytes", num_bytes_copied);

    Ok(())
}
