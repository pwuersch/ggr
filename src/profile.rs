use std::{path::PathBuf, str::FromStr};

use color_eyre::Result;
use serde::{Deserialize, Serialize};

const PROFILES_PATH: &str = ".config/profiles.json";
const APP_VERSION: &str = clap::crate_version!();

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileManager {
    pub profiles: Vec<Profile>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub git: GitProfile,
    pub api: ApiProfile,
}

impl ProfileManager {
    pub fn new() -> ProfileManager {
        ProfileManager::default()
    }

    pub async fn add_default_profiles(&mut self) -> Result<()> {
        self.profiles.push(Profile {
            name: "GitLab".to_string(),
            git: GitProfile {
                commit_name: "Philipp".to_string(),
                commit_email: "philipp@wuersch.org".to_string(),
            },
            api: ApiProfile {
                adapter: ApiAdapter::GitHub,
            },
        });
        Ok(())
    }

    pub async fn load_profiles(&mut self) -> Result<()> {
        let mut path = PathBuf::from_str(&std::env::var("HOME")?)?;
        path.push(PROFILES_PATH);

        let file_content = tokio::fs::read_to_string(path).await?;
        let profiles: SaveProfile = serde_json::from_str(&file_content)?;
        self.profiles = profiles.profiles;

        Ok(())
    }

    pub fn find_profile_by_name(&self, profile_name: &str) -> Option<&Profile> {
        self.profiles
            .iter()
            .find(|profile| profile.name == profile_name)
    }

    pub fn get_profile_names(&self) -> Vec<String> {
        self.profiles
            .iter()
            .map(|profile| profile.name.clone())
            .collect()
    }

    pub async fn save_profiles(&self) -> Result<()> {
        let save_profile = SaveProfile::new(self.profiles.to_vec());
        let save_text = serde_json::to_string_pretty(&save_profile)?;

        let mut save_path = PathBuf::from_str(&std::env::var("HOME")?)?;
        save_path.push(PROFILES_PATH);
        let result = tokio::fs::write(save_path, save_text).await;
        match result {
            Ok(_) => (),
            Err(err) => _ = dbg!(err),
        }

        Ok(())
    }
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}: ({}, {})",
            self.name, self.git.commit_name, self.git.commit_email
        )
    }
}

impl SaveProfile {
    pub fn new(profiles: Vec<Profile>) -> SaveProfile {
        SaveProfile {
            profiles,
            version: APP_VERSION.to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct SaveProfile {
    pub profiles: Vec<Profile>,
    pub version: String,
}

impl Default for ProfileManager {
    fn default() -> Self {
        ProfileManager {
            profiles: Vec::with_capacity(8),
        }
    }
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct GitProfile {
    pub commit_name: String,
    pub commit_email: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiProfile {
    pub adapter: ApiAdapter,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum ApiAdapter {
    GitLab,
    GitHub,
}
