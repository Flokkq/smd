use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::{
    configuration::configuration::APIConfiguration,
    skeleton::{Skeleton, SkeletonSyntax},
};

#[derive(Debug, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Template {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateInfo {
    #[serde(rename = "available")]
    pub available: String,
    #[serde(rename = "currentVersion")]
    pub current_version: String,
    #[serde(rename = "latestVersion")]
    pub latest_version: String,
    #[serde(rename = "downloadUrl")]
    pub download_url: String,
    #[serde(rename = "releaseNotes")]
    pub release_notes: String,
}

pub struct APICommunicator {
    configuration: APIConfiguration,
    communicator: Client,
}

impl APICommunicator {
    pub fn build(configuration: APIConfiguration) -> Self {
        Self {
            configuration,
            communicator: Client::builder().build().unwrap(),
        }
    }

    fn application_url(&self) -> String {
        format!(
            "http://{}:{}",
            self.configuration.host, self.configuration.port
        )
    }

    pub async fn post_markdown(&self, body: &str) -> Result<String> {
        let url = format!("{}/render", self.application_url());
        let response = self
            .communicator
            .post(&url)
            .header("Content-Type", "text/plain")
            .body(body.to_string())
            .send()
            .await?;

        response.error_for_status_ref()?;

        Ok(response.text().await?)
    }

    pub async fn get_skeleton(
        &self,
        skeleton: &str,
    ) -> Result<Skeleton, anyhow::Error> {
        let url = format!("{}/skeletons/{}", self.application_url(), skeleton);
        let response = self.communicator.get(&url).send().await?;

        response.error_for_status_ref()?;

        let syntax =
            SkeletonSyntax::from_json(response.json().await.unwrap()).unwrap();

        Ok(Skeleton::new(syntax))
    }

    pub async fn get_skeletons(&self) -> Result<Vec<Template>, anyhow::Error> {
        let url = format!("{}/skeletons/", self.application_url());
        let response = self.communicator.get(&url).send().await?;

        response.error_for_status_ref()?;
        Ok(response.json::<Vec<Template>>().await?)
    }

    pub async fn get_flavour(&self, flavour: &str) -> Result<String> {
        let url = format!("{}/themes/{}", self.application_url(), flavour);
        let response = self.communicator.get(&url).send().await?;

        response.error_for_status_ref()?;

        Ok(response.text().await?)
    }

    pub async fn get_flavours(&self) -> Result<Vec<Theme>, anyhow::Error> {
        let url = format!("{}/themes/", self.application_url());
        let response = self.communicator.get(&url).send().await?;

        response.error_for_status_ref()?;
        Ok(response.json::<Vec<Theme>>().await?)
    }

    pub async fn get_update(
        &self,
        version: &str,
        os: &str,
        cpu: &str,
    ) -> Result<UpdateInfo, anyhow::Error> {
        let url = format!("{}/update", self.application_url());
        let response = self
            .communicator
            .get(&url)
            .header("X-Client-Version", version)
            .header("X-Client-OS", os)
            .header("X-Client-CPU", cpu)
            .send()
            .await?;

        response.error_for_status_ref()?;

        Ok(response.json::<UpdateInfo>().await?)
    }
}
