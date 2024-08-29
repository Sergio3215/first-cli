mod cli;
mod echo;
use structopt::StructOpt;

use cli::{Action::*, CommandLineArgs};


fn main() -> anyhow::Result<()> {
    let CommandLineArgs {
        action,
    } = cli::CommandLineArgs::from_args();
    // println!("{:#?}", cli::CommandLineArgs::from_args());

    match action{
        Echo {message} => echo::EchoCommands::exec(message),
    };
    Ok(())
}
