/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use serde::Deserialize;
use std::process::Command;

use crate::error::Error;
use crate::error_kind::SECRETS_MANAGER_FAILURE;
use crate::secrets::secrets_manager::SecretsManager;

#[derive(Deserialize)]
struct BitwardenSecret {
    pub id: String,
    pub organizationId: String,
    pub projectId: String,
    pub key: String,
    pub value: String,
    pub creationDate: String,
    pub revisionDate: String,
}

pub struct BitwardenSecretsManager {
    access_token: String,
}

impl BitwardenSecretsManager {
    pub fn new(access_token: String) -> Self {
        Self { access_token }
    }
}

impl SecretsManager for BitwardenSecretsManager {
    fn get_secret(&self, secret_id: &str) -> Result<String, Error> {
        let result = Command::new("bws")
            .arg("get")
            .arg("secret")
            .arg(secret_id)
            .arg("--access-token")
            .arg(&self.access_token)
            .output();

        let secret = match result {
            Ok(output) => {
                if output.status.success() {
                    match serde_json::from_slice::<BitwardenSecret>(output.stdout.as_slice()) {
                        Ok(secret) => secret.value,
                        Err(error) => return Err(error.into()),
                    }
                } else {
                    return match String::from_utf8(output.stderr) {
                        Ok(error_message) => Err(Error::new(
                            SECRETS_MANAGER_FAILURE.to_string(),
                            format!("failed to run 'bws': {}", error_message),
                        )),
                        Err(error) => Err(Error::new(
                            SECRETS_MANAGER_FAILURE.to_string(),
                            format!("failed to read 'bws' error: {}", error),
                        )),
                    };
                }
            }
            Err(error) => return Err(error.into()),
        };

        Ok(secret)
    }
}
