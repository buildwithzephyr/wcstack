use clap::Parser;
use wcstack::{commands::Cli, errors::WcStackError, store::Store};

fn main() -> Result<(), WcStackError> {
    let cli = Cli::parse();
    let store = Store::new_in_current_workspace()?;
    cli.command.execute(store)?;

    Ok(())
}
