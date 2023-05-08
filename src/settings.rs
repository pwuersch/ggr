use color_eyre::{eyre::eyre, Result};

use crate::profile::ProfileManager;

pub struct SettingsActions;

impl SettingsActions {
    pub async fn list() -> Result<()> {
        let mut manager = ProfileManager::new();
        manager.load_profiles().await?;
        manager
            .profiles
            .iter()
            .for_each(|profile| println!("{}", profile));

        manager.save_profiles().await?;
        Ok(())
    }

    pub async fn add() -> Result<()> {
        let mut manager = ProfileManager::new();
        // ignore failed load
        _ = manager.load_profiles().await;
        manager.add_profile_interactively()?;
        manager.save_profiles().await?;

        Ok(())
    }

    pub async fn remove() -> Result<()> {
        let mut manager = ProfileManager::new();
        manager.load_profiles().await?;
        if manager.profiles.is_empty() {
            return Err(eyre!("No profiles were found"));
        }
        manager.remove_profile_interactively()?;
        manager.save_profiles().await?;

        Ok(())
    }
}
