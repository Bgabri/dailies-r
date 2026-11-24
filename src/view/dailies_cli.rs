use clap::{Parser, Subcommand};

/// A, tree based, task tracking and management system.
#[derive(Parser, Debug)]
#[command(version, about,after_help = "Controls:
  arrow keys, tab  navigation
  q                quit
  a                add task
  e                edit task
  d                delete task
  c                complete task
  space            track task
  enter            confirm selection
  esc              cancel selection"
)]
pub struct DailiesArgs {
    #[command(subcommand)]
    pub command: Option<Subcommands>,

    /// the refresh rate of the application in milliseconds
    #[arg(short, long, default_value_t = 1_000)]
    pub refresh: u64,

    /// the day to generate the report for
    #[arg(short, long)]
    pub day: Option<String>,

    /// disables auto tracking of the tasks
    #[arg(short, long)]
    pub no_auto_track: bool,

    /// the number of seconds to wait before starting or stopping a task session
    #[arg(short, long, default_value_t = if cfg!(debug_assertions) { 5 } else { 60 })]
    pub grace: u32,

    /// the downtime length before excluding it from tracking, in minutes
    #[arg(short = 'c', long, default_value_t = 5)]
    pub downtime_interval: u32,
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
