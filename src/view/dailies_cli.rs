use clap::{Parser, Subcommand};


/// A tree based task tracking and management system
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct DailiesArgs {
    #[command(subcommand)]
    pub command: Option<Subcommands>,
}

#[derive(Subcommand, Debug)]
pub enum Subcommands {
    /// starts the task manager in interactive mode
    Interactive,
    /// lists all detected open windows
    Windows,
}

impl DailiesArgs {
    pub fn get_args() -> DailiesArgs {
        Self::parse()
    }
}
