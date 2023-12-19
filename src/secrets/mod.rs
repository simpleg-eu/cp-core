/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::sync::Arc;

use crate::error::Error;
use crate::error_kind::SECRETS_MANAGER_FAILURE;
use crate::secrets::bitwarden_secrets_manager::BitwardenSecretsManager;
use crate::secrets::secrets_manager::SecretsManager;

mod bitwarden_secrets_manager;
pub mod secrets_manager;

pub fn get_secrets_manager() -> Result<Arc<dyn SecretsManager + Send + Sync>, Error> {
    let access_token = match std::env::var("SECRETS_MANAGER_ACCESS_TOKEN") {
        Ok(access_token) => access_token,
        Err(error) => {
            return Err(Error::new(
                SECRETS_MANAGER_FAILURE.to_string(),
                format!(
                    "failed to retrieve secrets manager access token from environment variable: {}",
                    error
                ),
            ))
        }
    };

    Ok(Arc::new(BitwardenSecretsManager::new(access_token)))
}
