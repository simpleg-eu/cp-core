use std::{env, time::Duration};

use crate::{
    config::{
        client::Client, file_getter::FileGetter, http_downloader::HttpDownloader,
        zip_extractor::ZipExtractor,
    },
    error::Error,
    error_kind::NOT_FOUND,
};

const CP_CONFIG_ACCESS_TOKEN_ENV: &str = "CP_CONFIG_ACCESS_TOKEN";

pub fn build(
    access_token: String,
    host: String,
    stage: String,
    environment: String,
    component: String,
    working_path: String,
    download_timeout: Duration,
) -> Result<Client<HttpDownloader, ZipExtractor, FileGetter>, Error> {
    let http_client = reqwest::Client::new();

    let downloader = HttpDownloader::new(access_token, download_timeout, http_client);
    let zip_extractor = ZipExtractor::default();
    let file_getter = FileGetter::new(working_path.clone());

    let client = Client::new(
        host,
        stage,
        environment,
        component,
        working_path,
        downloader,
        zip_extractor,
        file_getter,
    );

    Ok(client)
}
