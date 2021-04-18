use anyhow::Result;
use clap::ArgMatches;
use crate::ConnectionRepository;
use crate::connection::Server;
use colored::*;
use anyhow::Context;

async fn list(connections: &mut ConnectionRepository) -> Result<()> {
    // println!("ALL CONNECTIONS");

    let server_list = connections.list_connections().await?;
    // TODO Move print logic into server type itself
    let header = format!("{: <10}\t{: <70}", "Name", "Host");
    println!("{}", header.bold());
    for server in server_list {
        // let read_only = if server.read_only { "Y" } else { "N" };
        // println!("{: <10}\t{: <10}", "localhost:27017", read_only);
        println!("{}", server);
    }

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
