use anyhow::Result;
use anyhow::Context;
use crate::ConnectionRepository;
use crate::Server;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct ConnectionAddCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for ConnectionAddCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let name = args.value_of("name")
            .with_context(|| "No connection name provided")?;
        let info = Server::prompt_details(name)?;
        self.connections.add_connection(&info)?;

        Ok(())
    }
}
