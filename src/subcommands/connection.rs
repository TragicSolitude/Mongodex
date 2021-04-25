use anyhow::Result;
use clap::ArgMatches;
use crate::ConnectionRepository;
use crate::connection::Server;
use anyhow::Context;

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    match args.subcommand() {
        ("list", Some(_subargs)) => {
            let server_list = connections.list_connections().await?;
            println!("{}", server_list);
        },
        ("add", Some(subargs)) => {
            let name = subargs.value_of("name")
                .with_context(|| "Name argument not provided")?;

            let info = Server::prompt_details(name)?;
            connections.add_connection(&info).await?;
        },
        ("remove", Some(subargs)) => {
            let name = subargs.value_of("name")
                .with_context(|| "Name argument not provided")?;

            connections.remove_connection(name).await?;
        },
        ("edit", Some(subargs)) => {
            let name = subargs.value_of("name")
                .with_context(|| "Name argument not provided")?;

            let mut connection = connections.get_connection(name).await?;
            connection.prompt_update_details()?;
            connections.replace_connection(&connection).await?;
        },
        _ => {}
    };

    Ok(())
}
