/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::process::Command;

use serde::Deserialize;

use crate::error::Error;
use crate::error_kind::SECRETS_MANAGER_FAILURE;
use crate::secrets::secrets_manager::SecretsManager;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct BitwardenSecret {
    pub id: String,
    pub organization_id: String,
    pub project_id: String,
    pub key: String,
    pub value: String,
    pub creation_date: String,
    pub revision_date: String,
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

#[cfg(test)]
pub mod tests {
    use crate::config_reader::ConfigReader;
    use crate::secrets::bitwarden_secrets_manager::BitwardenSecretsManager;
    use crate::secrets::secrets_manager::SecretsManager;
    use crate::test_base::get_unit_test_data_path;

    #[test]
    fn get_secret_existing_secret_returns_expected_string() {
        let expected_string: String = "le_secret :)".to_string();
        let config_reader = ConfigReader::default();
        let mut config_path = get_unit_test_data_path(file!());
        config_path.push("config.yaml");
        let config_result = config_reader.read(config_path).unwrap();
        let access_token = std::env::var("SECRETS_MANAGER_ACCESS_TOKEN").unwrap();
        let secret_id = config_result
            .get("ExampleSecret")
            .unwrap()
            .as_str()
            .unwrap();
        let secrets_manager: BitwardenSecretsManager = BitwardenSecretsManager::new(access_token);

        let result = secrets_manager.get_secret(secret_id).unwrap();

        assert_eq!(expected_string, result);
    }
}
