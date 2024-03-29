/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2024.
 */

use std::time::Duration;

use async_trait::async_trait;
use reqwest::Client;
use tokio::time::timeout;

use crate::config::downloader::Downloader;
use crate::error::Error;
use crate::error_kind::TIMED_OUT;

pub struct HttpDownloader {
    access_token: String,
    download_timeout: Duration,
    client: Client,
}

impl HttpDownloader {
    pub fn new(access_token: String, download_timeout: Duration, client: Client) -> Self {
        Self {
            access_token,
            download_timeout,
            client,
        }
    }
}

#[async_trait]
impl Downloader for HttpDownloader {
    async fn download(
        &self,
        host: &str,
        stage: &str,
        environment: &str,
        component: &str,
    ) -> Result<Vec<u8>, Error> {
        let url = format!(
            "{}/config?stage={}&environment={}&component={}",
            host, stage, environment, component
        );

        let request_builder = self.client.get(url).bearer_auth(self.access_token.clone());

        let response = match timeout(self.download_timeout, request_builder.send()).await {
            Ok(result) => match result {
                Ok(response) => response,
                Err(error) => return Err(error.into()),
            },
            Err(_) => {
                return Err(Error::new(
                    TIMED_OUT,
                    "configuration download has timed out",
                ));
            }
        };

        let package_data = match response.bytes().await {
            Ok(package_data) => package_data,
            Err(error) => return Err(error.into()),
        };

        return Ok(package_data.to_vec());
    }
}

#[cfg(test)]
pub mod tests {
    use std::time::Duration;

    use reqwest::Client;
    use serde_yaml::Value;
    use tokio::time::Instant;

    use crate::config::downloader::Downloader;
    use crate::config::http_downloader::HttpDownloader;
    use crate::config_reader::ConfigReader;
    use crate::error_kind::TIMED_OUT;
    use crate::secrets::get_secrets_manager;
    use crate::test_base::get_unit_test_data_path;

    #[tokio::test]
    pub async fn download_valid_config_downloads_package() {
        let config = get_config();
        let access_token_secret = config.get("AccessTokenSecret").unwrap().as_str().unwrap();
        let access_token = get_secrets_manager()
            .unwrap()
            .get_secret(access_token_secret)
            .unwrap();
        let host = config.get("Host").unwrap().as_str().unwrap();
        let stage = config.get("Stage").unwrap().as_str().unwrap();
        let environment = config.get("Environment").unwrap().as_str().unwrap();
        let component = config.get("Component").unwrap().as_str().unwrap();
        let download_timeout = Duration::from_secs(
            config
                .get("DownloadTimeoutInSeconds")
                .unwrap()
                .as_u64()
                .unwrap(),
        );
        let downloader = HttpDownloader::new(access_token, download_timeout, Client::new());

        let result = downloader
            .download(host, stage, environment, component)
            .await;

        assert!(result.is_ok());
        assert!(result.unwrap().len() > 0);
    }

    fn get_config() -> Value {
        let config_reader = ConfigReader::default();
        let mut config_path = get_unit_test_data_path(file!());
        config_path.push("config.yaml");
        let config = config_reader
            .read(config_path)
            .expect("expected configuration file");

        return config;
    }
}
