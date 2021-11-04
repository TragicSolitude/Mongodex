use std::io;
use anyhow::Result;
use anyhow::Context;
use crate::ConnectionRepository;
use crate::Database;
use super::Command;

#[derive(shaku::Provider)]
#[shaku(interface = Command)]
pub struct MigrateCommand {
    #[shaku(provide)]
    connections: Box<dyn ConnectionRepository>
}

impl Command for MigrateCommand {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()> {
        let source_name = args.value_of("source")
            .with_context(|| "Source not provided")?;
        let destination_name = args.value_of("destination")
            .with_context(|| "Destination not provided")?;

        let source = Database::from_str(self.connections.as_mut(), source_name)?;
        let destination = Database::from_str(self.connections.as_mut(), destination_name)?;

        if destination.read_only {
            return Err(anyhow!("Destination is read only"));
        }

        let num_bytes_copied = {
            let source_db_name = Some(source.db_name.as_str());
            let mut destination_guardian = destination.restore(source_db_name)?;
            let mut source_guardian = source.dump()?;

            io::copy(source_guardian.output(), destination_guardian.input())?
        };
        println!("Migrated {} bytes", num_bytes_copied);

        Ok(())
    }
}
