use clap::{arg, command, Parser, Subcommand};
use color_eyre::Result;
use serde::{Deserialize, Serialize};
use std::{process::Stdio, str};
use tokio::process::Command;

#[derive(Parser)]
#[command(name = "Git CLI")]
#[command(author = "Philipp Würsch")]
#[command(version = "1.0")]
#[command(about = "Does cool things with git repos", long_about = None)]
struct Cli {
    #[arg(help = "Enables verbose logging (debug output)")]
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Clones a repository")]
    Clone {
        #[arg(help = "Sets the remote url")]
        #[arg(short, long)]
        url: String,

        #[arg(help = "The destination directory")]
        #[arg(short, long)]
        directory: Option<String>,
    },
    #[command(about = "Configures a repository with one of your profiles")]
    Config {
        #[arg(help = "The name of the profile to use")]
        #[arg(short, long)]
        profile: Option<String>,
    },
    #[command(about = "Commits all local changes")]
    Commit {
        #[arg(help = "The commit message")]
        #[arg(short, long)]
        message: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Clone { url, directory } => clone(url, directory).await?,
        Commands::Config { profile } => config(profile).await?,
        Commands::Commit { message } => commit(message).await?,
    };

    Ok(())
}

async fn clone(url: String, directory: Option<String>) -> Result<()> {
    new_git_command(vec![
        "clone",
        &url,
        &directory.unwrap_or("./destination".to_string()),
    ])
    .spawn()?
    .wait()
    .await?;

    Ok(())
}

async fn config(profile: Option<String>) -> Result<()> {
    let mut manager = ProfileManager::new();
    manager.load_profiles().await?;
    let profile = manager.find_profile_by_name(&profile.unwrap_or("Test".to_string()));
    dbg!(profile);

    Ok(())
}

async fn commit(message: Option<String>) -> Result<()> {
    let message = match message {
        Some(message) => message,
        None => "chore: some default message".to_string(), // TODO: get interactive message input
    };

    let mut command = new_git_command(vec!["commit", "-m", &message]);
    command.spawn()?.wait().await?;

    Ok(())
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileManager {
    profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Profile {
    name: String,
    git: GitProfile,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct GitProfile {
    commit_name: String,
    commit_email: String,
}

impl ProfileManager {
    pub fn new() -> ProfileManager {
        ProfileManager::default()
    }

    pub async fn load_profiles(&mut self) -> Result<()> {
        self.profiles.push(Profile {
            name: "GitLab".to_string(),
            git: GitProfile {
                commit_name: "Philipp".to_string(),
                commit_email: "philipp@wuersch.org".to_string(),
            },
        });
        Ok(())
    }

    pub fn find_profile_by_name(&self, profile_name: &str) -> Option<&Profile> {
        self.profiles
            .iter()
            .find(|profile| profile.name == profile_name)
    }

    pub async fn save_profiles() -> Result<()> {
        Ok(())
    }
}

impl Default for ProfileManager {
    fn default() -> Self {
        ProfileManager {
            profiles: Vec::with_capacity(8),
        }
    }
}

fn new_git_command(args: Vec<&str>) -> Command {
    let mut command = Command::new("git");
    command.stdin(Stdio::null());
    command.stdout(Stdio::inherit());
    args.iter().for_each(|arg| _ = command.arg(arg));
    command
}
