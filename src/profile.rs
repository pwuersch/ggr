use color_eyre::{eyre::eyre, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use strum::{EnumIter, IntoEnumIterator};

const PROFILES_PATH: &str = ".config/profiles.json";
const APP_VERSION: &str = clap::crate_version!();

#[derive(Serialize, Deserialize, Debug)]
pub struct ProfileManager {
    pub profiles: Vec<Profile>,
}

impl ProfileManager {
    pub fn new() -> ProfileManager {
        ProfileManager::default()
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

    pub fn get_profile_interactively(&self) -> Result<Profile> {
        let profile =
            inquire::Select::new("Select a profile from the list:", self.profiles.clone())
                .prompt()?;
        Ok(profile)
    }

    pub fn remove_profile_interactively(&mut self) -> Result<()> {
        let profile = self.get_profile_interactively()?;
        let profile_index = self
            .profiles
            .iter()
            .position(|current_profile| current_profile.name == profile.name);

        match profile_index {
            None => {
                return Err(eyre!(
                    "INTERNAL ERROR: was not able to find the index of the selected profile"
                ))
            }
            Some(profile_index) => self.profiles.remove(profile_index),
        };

        Ok(())
    }

    pub fn add_profile_interactively(&mut self) -> Result<()> {
        let mut new_profile = Profile {
            name: String::default(),
            git: GitProfile {
                ..Default::default()
            },
            api_adapter: None,
        };

        new_profile.name = inquire::Text::new("Enter a name for your new profile:")
            .with_validator(get_non_empty_validator())
            .prompt()?;
        new_profile.git.commit_name = inquire::Text::new("Enter the commit name for your profile:")
            .with_validator(get_non_empty_validator())
            .prompt()?;
        new_profile.git.commit_email =
            inquire::Text::new("Enter the commit email for your profile:")
                .with_validator(get_non_empty_validator())
                .prompt()?;

        let available_api_adapters: Vec<ApiAdapter> = ApiAdapter::iter().collect();
        let available_api_adapters_list = available_api_adapters
            .iter()
            .map(|adapter| adapter.to_string())
            .collect::<Vec<String>>()
            .join(", ");

        let enable_api_adapter = inquire::Confirm::new(&format!(
            "Do you want to enable an API adapter for your profile? ({})",
            available_api_adapters_list
        ))
        .with_default(true)
        .prompt()?;

        if enable_api_adapter {
            let api_adapter =
                inquire::Select::new("Select one of the API adapters:", available_api_adapters)
                    .prompt()?;

            match api_adapter {
                ApiAdapter::GitHub { token: _ } => {
                    let token = inquire::Password::new("Enter the GitHub personal access token:")
                        .with_validator(get_non_empty_validator())
                        .prompt()?;
                    new_profile.api_adapter = Some(ApiAdapter::GitHub { token });
                }
                ApiAdapter::GitLab { token: _, host: _ } => {
                    let host = inquire::Text::new("Enter the GitLab instance host:")
                        .with_validator(get_non_empty_validator())
                        .with_default("gitlab.com")
                        .prompt()?;
                    let token = inquire::Password::new("Enter the GitLab access token:")
                        .with_validator(get_non_empty_validator())
                        .prompt()?;

                    new_profile.api_adapter = Some(ApiAdapter::GitLab { token, host });
                }
            };
        }
        self.profiles.push(new_profile.clone());
        println!("Successfully added new profile: {}", new_profile);

        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Profile {
    pub name: String,
    pub git: GitProfile,
    pub api_adapter: Option<ApiAdapter>,
}

impl std::fmt::Display for Profile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.api_adapter {
            Some(api_adapter) => match api_adapter {
                ApiAdapter::GitHub { token: _ } => write!(
                    f,
                    "{}: Git ({}, {}) GitHub (token: ****) ",
                    self.name, self.git.commit_name, self.git.commit_email
                ),
                ApiAdapter::GitLab { token: _, host } => write!(
                    f,
                    "{}: Git ({}, {}) GitLab (host: {}, token: ****)",
                    self.name, self.git.commit_name, self.git.commit_email, host
                ),
            },
            None => write!(
                f,
                "{}: Git ({}, {})",
                self.name, self.git.commit_name, self.git.commit_email
            ),
        }
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

#[derive(Serialize, Deserialize, Debug, Clone, EnumIter, strum::Display)]
pub enum ApiAdapter {
    GitLab { token: String, host: String },
    GitHub { token: String },
}

fn get_non_empty_validator() -> inquire::validator::ValueRequiredValidator {
    inquire::validator::ValueRequiredValidator::new("Required input")
}
