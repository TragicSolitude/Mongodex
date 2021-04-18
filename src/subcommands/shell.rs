use clap::ArgMatches;
use crate::connection::ConnectionRepository;
use anyhow::Result;
use anyhow::Context;
use anyhow::Error;

pub async fn run<'a, 'b>(connections: &'a mut ConnectionRepository, args: &'b ArgMatches<'b>) -> Result<()> {
    let name = args.value_of("connection_name")
        .with_context(|| "Name argument not provided")?;

    let server = connections.get_connection(name).await?;
    // This is a bit of a special function in that it only returns if there is
    // an error. This is why the code might look a little weird where it seems
    // to always error. On success the shell() function (which uses execvp
    // internally) does not return.
    let error = server.shell();

    Err(Error::from(error))
}
