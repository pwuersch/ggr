use color_eyre::{eyre::eyre, Result};

use crate::{api::ApiAdapter, new_git_command, profile::ProfileManager};

pub struct GitActions;

impl GitActions {
    pub async fn clone(_url: Option<String>, _directory: Option<String>) -> Result<()> {
        let mut manager = ProfileManager::new();
        manager.load_profiles().await?;
        let profile = manager.get_profile_interactively()?;
        if let Some(api_adapter) = profile.api_adapter {
            let repos = api_adapter.get_repos().await?;
            dbg!(repos);
        }

        // new_git_command(&[
        //     "clone",
        //     &url,
        //     &directory.unwrap_or("./destination".to_string()),
        // ])
        // .spawn()?
        // .wait()
        // .await?;

        Ok(())
    }

    pub async fn config(profile: Option<String>) -> Result<()> {
        let mut manager = ProfileManager::new();
        manager.load_profiles().await?;

        let profile = match profile {
            None => manager.get_profile_interactively()?,
            Some(profile_name) => {
                if let Some(profile) = manager.find_profile_by_name(&profile_name) {
                    profile.clone()
                } else {
                    return Err(eyre!("No profile with the name {} found", profile_name));
                }
            }
        };

        println!("Using profile {}", profile);

        Ok(())
    }

    pub async fn commit(message: Option<String>) -> Result<()> {
        let message = match message {
            Some(message) => message,
            None => inquire::Text::new("Enter your commit message:").prompt()?,
        };

        _ = new_git_command(&["add", "."]).spawn()?.wait().await?;
        _ = new_git_command(&["commit", "-m", &message])
            .spawn()?
            .wait()
            .await?;

        Ok(())
    }
}
