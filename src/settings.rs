use color_eyre::Result;

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
        _ = manager.load_profiles().await;
        manager.add_default_profiles().await?;
        manager.save_profiles().await?;

        Ok(())
    }

    pub async fn remove() -> Result<()> {
        let mut manager = ProfileManager::new();
        manager.load_profiles().await?;
        manager.save_profiles().await?;

        Ok(())
    }
}
