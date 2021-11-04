use anyhow::Result;
use anyhow::Context;
use crate::RUNTIME;
use crate::ConnectionRepository;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct ListdatabasesCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

// TODO fails to connect to local database even when simple mongo command works
impl Command for ListdatabasesCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let name = args.value_of("name")
            .with_context(|| "Name argument not provided")?;

        let server = self.connections.get_connection(name)?;
        let server_connection = server.connect()?;

        let databases = RUNTIME.block_on(
            server_connection.list_database_names(None, None))?;

        for database_name in databases {
            println!("{}", database_name);
        }

        Ok(())
    }
}
