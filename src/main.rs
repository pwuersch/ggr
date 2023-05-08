mod git;
mod profile;
mod settings;

use crate::{git::GitActions, settings::SettingsActions};
use clap::{arg, command, Parser, Subcommand};
use color_eyre::Result;
use std::{process::Stdio, str};
use tokio::process::Command;

#[derive(Parser)]
#[command(name = "Git CLI")]
#[command(author = clap::crate_authors!())]
#[command(version = clap::crate_version!())]
#[command(about = clap::crate_description!(), long_about = None)]
struct Cli {
    #[arg(short, long, global = true)]
    /// Enables verbose logging (debug output)
    verbose: bool,

    #[command(subcommand)]
    command: CliCommands,
}

#[derive(Subcommand)]
enum CliCommands {
    /// Clones a repository
    Clone {
        /// Sets the remote url
        #[arg(short, long)]
        url: String,

        /// The destination directory
        #[arg(short, long)]
        directory: Option<String>,
    },
    /// Configures a repository with one of your profiles
    Config {
        /// The name of the profile to use
        #[arg(short, long)]
        profile: Option<String>,
    },
    /// Commits all local changes
    Commit {
        /// The commit message
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Manages the local profiles
    Settings(Settings),
}

#[derive(Parser)]
struct Settings {
    #[command(subcommand)]
    settings_commands: SettingsCommands,
}

#[derive(Subcommand)]
enum SettingsCommands {
    /// Lists all the profiles
    List,
    /// Adds a new profile
    Add,
    /// Removes an existing profile
    Remove,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        CliCommands::Clone { url, directory } => GitActions::clone(url, directory).await?,
        CliCommands::Config { profile } => GitActions::config(profile).await?,
        CliCommands::Commit { message } => GitActions::commit(message).await?,
        CliCommands::Settings(settings) => match settings.settings_commands {
            SettingsCommands::List => SettingsActions::list().await?,
            SettingsCommands::Add => SettingsActions::add().await?,
            SettingsCommands::Remove => SettingsActions::remove().await?,
        },
    };

    Ok(())
}

fn new_git_command(args: Vec<&str>) -> Command {
    let mut command = Command::new("git");
    command.stdin(Stdio::null());
    command.stdout(Stdio::inherit());
    args.iter().for_each(|arg| _ = command.arg(arg));
    command
}
