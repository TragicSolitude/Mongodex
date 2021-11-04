use anyhow::Result;
use crate::ConnectionRepository;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct ConnectionListCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for ConnectionListCommand {
    fn run(&mut self, _args: &clap::ArgMatches) -> Result<()> {
        let server_list = self.connections.list_connections()?;
        println!("{}", server_list);

        Ok(())
    }
}
