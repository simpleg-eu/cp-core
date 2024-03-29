use async_trait::async_trait;

use crate::error::Error;

#[async_trait]
/// `Downloader` provides the ability to download a configuration from a remote site.
pub trait Downloader {
    ///
    /// `Download` downloads the latest configuration from a specific remote site.
    ///
    /// # Arguments
    ///
    /// * `host` - URL of the configuration provider.
    /// * `stage` - flavour of the configuration being retrieved. For now the git branch from which
    /// the configuration is downloaded.
    /// * `environment` - i.e. 'development', 'staging' and 'production'.
    /// * `component` - microservice for which the configuration package is being downloaded.
    ///
    /// # Returns
    ///
    /// * __Ok__(`Vec<u8>`) - configuration package's bytes.
    /// * __Err__(`Error`) - error indicating what went wrong.
    async fn download(
        &self,
        host: &str,
        stage: &str,
        environment: &str,
        component: &str,
    ) -> Result<Vec<u8>, Error>;
}
