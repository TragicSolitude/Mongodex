use anyhow::Result;
use anyhow::Context;
use crate::ConnectionRepository;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct ConnectionRemoveCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for ConnectionRemoveCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let name = args.value_of("name")
            .with_context(|| "No connection name provided")?;
        self.connections.remove_connection(name)?;

        Ok(())
    }
}
