use color_eyre::Result;

use crate::{new_git_command, profile::ProfileManager};

pub struct GitActions;

impl GitActions {
    pub async fn clone(url: String, directory: Option<String>) -> Result<()> {
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

    pub async fn config(profile: Option<String>) -> Result<()> {
        let mut manager = ProfileManager::new();
        manager.load_profiles().await?;

        let profile_name = match profile {
            None => inquire::Select::new(
                "Select the Profile you with to use:",
                manager.get_profile_names(),
            )
            .prompt()?,
            Some(name) => name,
        };

        let profile = manager.find_profile_by_name(&profile_name);
        match profile {
            Some(profile) => println!("Selected profile: {}", profile),
            None => println!("No profile with that name found"),
        }

        Ok(())
    }

    pub async fn commit(message: Option<String>) -> Result<()> {
        let message = match message {
            Some(message) => message,
            None => "chore: some default message".to_string(), // TODO: get interactive message input
        };

        let mut command = new_git_command(vec!["commit", "-m", &message]);
        command.spawn()?.wait().await?;

        Ok(())
    }
}
