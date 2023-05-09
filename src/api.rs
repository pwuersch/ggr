use async_trait::async_trait;
use color_eyre::Result;
use reqwest::header::{ACCEPT, AUTHORIZATION, USER_AGENT};

const APP_USER_AGENT: &str = "reqwest/1.0.0";

#[async_trait]
pub trait ApiAdapter {
    async fn get_repos(&self) -> Result<Vec<Repo>>;
}

#[derive(Debug)]
pub struct Repo {
    pub url: String,
    pub display_name: String,
}

impl std::fmt::Display for Repo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({})", self.display_name, self.url)
    }
}

impl From<&github_types::Repository> for Repo {
    fn from(value: &github_types::Repository) -> Self {
        Repo {
            display_name: value.full_name.to_string(),
            url: value.url.to_string(),
        }
    }
}

#[async_trait]
impl ApiAdapter for crate::profile::ApiAdapter {
    async fn get_repos(&self) -> Result<Vec<Repo>> {
        Ok(match self {
            Self::GitHub { token } => GitHubAdapter::get_repos(token.to_string()).await?,
            Self::GitLab { token, host } => {
                GitLabAdapter::get_repos(token.to_string(), host.to_string()).await?
            }
        })
    }
}

struct GitHubAdapter;

impl GitHubAdapter {
    async fn get_repos(token: String) -> Result<Vec<Repo>> {
        let url = "https://api.github.com/user/repos?per_page=100&sort=updated";
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .header(AUTHORIZATION, format!("token {}", token))
            .header(ACCEPT, "application/vnd.github.v3+json")
            .header(USER_AGENT, APP_USER_AGENT)
            .send()
            .await?;

        let repos = response
            .json::<Vec<github_types::Repository>>()
            .await?
            .iter()
            .map(Repo::from)
            .collect();

        Ok(repos)
    }
}

mod git_hub_types {
    use serde::Deserialize;

    #[derive(Deserialize, Debug)]
    pub struct GitHubGetReposResponse {}
}

struct GitLabAdapter;

impl GitLabAdapter {
    async fn get_repos(token: String, host: String) -> Result<Vec<Repo>> {
        println!("token: {}, host: {}", token, host);
        Ok(vec![])
    }
}
