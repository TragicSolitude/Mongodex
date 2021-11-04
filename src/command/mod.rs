use anyhow::Result;
use shaku::ProviderFn;
use shaku::Provider;
use shaku::HasProvider;
use crate::ConnectionRepository;

pub trait Command {
    fn run(&mut self, args: &clap::ArgMatches) -> Result<()>;
}

impl Default for Box<dyn Command> {
    fn default() -> Self {
        Box::new(NullCommand)
    }
}

/// No-op command used to cover default cases
#[derive(shaku::Provider, Debug)]
#[shaku(interface = Command)]
pub struct NullCommand;
impl Command for NullCommand {
    fn run(&mut self, _args: &clap::ArgMatches) -> Result<()> {
        Ok(())
    }
}

mod connection_list;
mod connection_show;
mod connection_add;
mod connection_remove;
mod connection_edit;
mod listdatabases;
mod dump;
mod migrate;
mod restore;
mod shell;

pub fn get_command<'a, M>(args: &'a clap::ArgMatches<'a>) -> (ProviderFn<M, dyn Command>, &'a clap::ArgMatches)
where
    M: shaku::Module + HasProvider<dyn ConnectionRepository>
{
    use connection_list::ConnectionListCommand;
    use connection_add::ConnectionAddCommand;
    use connection_remove::ConnectionRemoveCommand;
    use connection_edit::ConnectionEditCommand;
    use connection_show::ConnectionShowCommand;
    use listdatabases::ListdatabasesCommand;
    use dump::DumpCommand;
    use migrate::MigrateCommand;
    use restore::RestoreCommand;
    use shell::ShellCommand;

    // That feels weird but I'll allow it
    //
    // TODO change this to `.provide()` the discrete command objects once better
    // support is available in shaku derive macro for "implementing" the same
    match args.subcommand() {
        ("connection", Some(args)) => match args.subcommand() {
            ("list", Some(args)) => (Box::new(ConnectionListCommand::provide), args),
            ("add", Some(args)) => (Box::new(ConnectionAddCommand::provide), args),
            ("remove", Some(args)) => (Box::new(ConnectionRemoveCommand::provide), args),
            ("edit", Some(args)) => (Box::new(ConnectionEditCommand::provide), args),
            ("show", Some(args)) => (Box::new(ConnectionShowCommand::provide), args),
            _ => (Box::new(NullCommand::provide), args)
        },
        ("listdatabases", Some(args)) => (Box::new(ListdatabasesCommand::provide), args),
        ("dump", Some(args)) => (Box::new(DumpCommand::provide), args),
        ("migrate", Some(args)) => (Box::new(MigrateCommand::provide), args),
        ("restore", Some(args)) => (Box::new(RestoreCommand::provide), args),
        ("shell", Some(args)) => (Box::new(ShellCommand::provide), args),
        _ => (Box::new(NullCommand::provide), args)
    }
}
