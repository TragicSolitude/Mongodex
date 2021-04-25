use anyhow::Result;
use clap::ArgMatches;
use crate::ConnectionRepository;
use crate::connection::Server;
use anyhow::Context;

async fn list(connections: &mut ConnectionRepository) -> Result<()> {
    let server_list = connections.list_connections().await?;
    println!("{}", server_list);

    Ok(())
}

async fn add<'a, 'b>(connections: &'a mut ConnectionRepository, name: &'b str) -> Result<()> {
    let info = Server::prompt_details(name)?;
    connections.add_connection(&info).await?;

    Ok(())
}

async fn remove<'a, 'b>(connections: &'a mut ConnectionRepository, name: &'b str) -> Result<()> {
    connections.remove_connection(name).await?;

    Ok(())
}

async fn edit<'a, 'b>(connections: &'a mut ConnectionRepository, name: &'b str) -> Result<()> {
    let mut connection = connections.get_connection(name).await?;
    connection.prompt_update_details()?;
    connections.replace_connection(&connection).await?;

    Ok(())
}

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    match args.subcommand() {
        ("list", Some(_args)) => list(connections).await,
        ("add", Some(args)) => {
            let name = args.value_of("name")
                .with_context(|| "Name argument not provided")?;

            add(connections, name).await
        },
        ("remove", Some(args)) => {
            let name = args.value_of("name")
                .with_context(|| "Name argument not provided")?;

            remove(connections, name).await
        },
        ("edit", Some(args)) => {
            let name = args.value_of("name")
                .with_context(|| "Name argument not provided")?;

            edit(connections, name).await
        },
        _ => Ok(())
    }
}
