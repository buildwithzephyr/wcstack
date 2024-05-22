use clap::Parser;
use std::process;
use wcstack::{commands::Cli, errors::WcStackError, store::Store};

// The type parameter T lets us call this in an unwrap_or_else. Never type (!) coerces to any type,
// but FnOnce -> ! does not coerce to any FnOnce.
fn exit_with_error<T>(error: impl Into<WcStackError>) -> T {
    eprintln!("{}", error.into().to_string());
    process::exit(1)
}

fn main() {
    let cli = Cli::parse();
    let store = Store::new_in_current_workspace().unwrap_or_else(exit_with_error);
    cli.command.execute(store).unwrap_or_else(exit_with_error);
}
