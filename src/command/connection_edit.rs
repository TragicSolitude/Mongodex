use anyhow::Result;
use anyhow::Context;
use crate::ConnectionRepository;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct ConnectionEditCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for ConnectionEditCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let name = args.value_of("name")
            .with_context(|| "No connection name provided")?;
        let mut connection = self.connections.get_connection(name)?;
        connection.prompt_update_details()?;
        self.connections.replace_connection(&connection)?;

        Ok(())
    }
}
