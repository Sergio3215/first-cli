// use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug,StructOpt)]
pub enum Action{
    ///Show a message of console
    Echo{
        #[structopt()]
        message:String
    },
}

#[derive(Debug,StructOpt)]
#[structopt(
    name="First CLI",
    about="My first CLI"
)]
pub struct CommandLineArgs{
    #[structopt(subcommand)]
    pub action:Action,
}