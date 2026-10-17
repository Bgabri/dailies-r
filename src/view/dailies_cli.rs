use clap::{Parser, Args, Subcommand, ValueEnum};


/// A tree based task tracking and management system
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DailiesArgs {
    #[command(subcommand)]
    pub command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    Interactive,
}

impl DailiesArgs {
    pub fn get_args() -> DailiesArgs {
        Self::parse()
    }
}
