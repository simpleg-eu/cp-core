use std::path::Path;

use serde::de::DeserializeOwned;

use crate::{
    config::{downloader::Downloader, extractor::Extractor, getter::Getter},
    error::Error,
};

/// `Client` facilitates the retrieval of configuration from a remote site without having to worry about
/// the internals of the process.
pub struct Client<
    TDownloader: Downloader + Send + Sync,
    TExtractor: Extractor + Send + Sync,
    TGetter: Getter + Send + Sync,
> {
    host: String,
    stage: String,
    environment: String,
    component: String,
    working_path: String,
    downloader: TDownloader,
    extractor: TExtractor,
    getter: TGetter,
}

impl<
        TDownloader: Downloader + Send + Sync,
        TExtractor: Extractor + Send + Sync,
        TGetter: Getter + Send + Sync,
    > Client<TDownloader, TExtractor, TGetter>
{
    pub fn new(
        host: String,
        stage: String,
        environment: String,
        component: String,
        working_path: String,
        downloader: TDownloader,
        extractor: TExtractor,
        getter: TGetter,
    ) -> Self {
        Self {
            host,
            stage,
            environment,
            component,
            working_path,
            downloader,
            extractor,
            getter,
        }
    }

    pub async fn get<T: DeserializeOwned>(
        &mut self,
        file_path: &str,
        key: &str,
    ) -> Result<T, Error> {
        // if directory does not exist, initialize configuration
        if std::fs::metadata(&self.working_path).is_err() {
            self.init_config().await?;
        }

        let value = self.getter.get::<T>(file_path, key)?;

        Ok(value)
    }

    async fn init_config(&self) -> Result<(), Error> {
        match std::fs::create_dir_all(&self.working_path) {
            Ok(_) => (),
            Err(error) => return Err(error.into()),
        }

        let config_package = self
            .downloader
            .download(&self.host, &self.stage, &self.environment, &self.component)
            .await?;

        self.extractor.extract(config_package, &self.working_path)?;

        Ok(())
    }
}

impl<
        TDownloader: Downloader + Send + Sync,
        TExtractor: Extractor + Send + Sync,
        TGetter: Getter + Send + Sync,
    > Drop for Client<TDownloader, TExtractor, TGetter>
{
    fn drop(&mut self) {
        match std::fs::remove_dir_all(Path::new(&self.working_path)) {
            Ok(_) => (),
            Err(error) => log::warn!("failed to remove working path: {}", error),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::config::client::Client;
    use crate::config::extractor::Extractor;
    use crate::config::getter::Getter;
    use crate::error_kind::NOT_IMPLEMENTED;
    use crate::{config::downloader::Downloader, error::Error};
    use async_trait::async_trait;
    use mockall::mock;
    use serde::de::DeserializeOwned;

    mock! {
        DownloaderStruct {}

        #[async_trait]
        impl Downloader for DownloaderStruct {
            async fn download(&self, host: &str, stage: &str, environment: &str, component: &str) -> Result<Vec<u8>, Error>;
        }
    }

    mock! {
        ExtractorStruct {}

        impl Extractor for ExtractorStruct {
            fn extract(&self, package_data: Vec<u8>, target_path: &str) -> Result<(), Error>;
        }
    }

    struct MockGetter {}

    impl MockGetter {
        pub fn new() -> Self {
            MockGetter {}
        }
    }

    impl Getter for MockGetter {
        fn get<T: DeserializeOwned>(&mut self, _: &str, _: &str) -> Result<T, Error> {
            Err(Error::new(NOT_IMPLEMENTED, "not implemented"))
        }
    }

    #[tokio::test]
    pub async fn get_initializes_configuration_if_working_path_does_not_exist() {
        let mut downloader = MockDownloaderStruct::new();
        let mut extractor = MockExtractorStruct::new();
        let getter = MockGetter::new();
        extractor.expect_extract().return_const(Ok(())).times(1);
        downloader
            .expect_download()
            .return_const(Ok(Vec::new()))
            .times(1);
        let mut client: Client<MockDownloaderStruct, MockExtractorStruct, MockGetter> = Client::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            downloader,
            extractor,
            getter,
        );

        let result = client.get::<bool>("", "").await;

        assert!(result.is_err());
        assert_eq!(NOT_IMPLEMENTED, result.unwrap_err().error_kind());
    }

    #[tokio::test]
    pub async fn get_does_not_initialize_if_working_path_exists() {
        let _ = std::fs::create_dir_all("example");
        let mut downloader = MockDownloaderStruct::new();
        let mut extractor = MockExtractorStruct::new();
        let getter = MockGetter::new();
        extractor.expect_extract().return_const(Ok(())).times(0);
        downloader
            .expect_download()
            .return_const(Ok(Vec::new()))
            .times(0);
        let mut client: Client<MockDownloaderStruct, MockExtractorStruct, MockGetter> = Client::new(
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "".to_string(),
            "example".to_string(),
            downloader,
            extractor,
            getter,
        );

        let result = client.get::<bool>("", "").await;

        let _ = std::fs::remove_dir_all("example");
        assert!(result.is_err());
        assert_eq!(NOT_IMPLEMENTED, result.unwrap_err().error_kind());
    }
}
