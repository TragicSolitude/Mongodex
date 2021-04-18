use clap::ArgMatches;
use crate::connection::ConnectionRepository;
use anyhow::Result;
use anyhow::Context;

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    let name = args.value_of("name")
        .with_context(|| "Name argument not provided")?;

    let server = connections.get_connection(name).await?;
    let server_connection = server.connect()?;

    let databases = server_connection.list_database_names(None, None).await?;

    for database_name in databases {
        println!("{}", database_name);
    }

    Ok(())
}
