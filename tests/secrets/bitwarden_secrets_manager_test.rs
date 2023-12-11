/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use crate::test_base::get_test_data_path;
use cp_core::config_reader::ConfigReader;
use cp_core::secrets::bitwarden_secrets_manager::BitwardenSecretsManager;
use cp_core::secrets::secrets_manager::SecretsManager;

#[test]
fn get_secret_existing_secret_returns_expected_string() {
    let expected_string: String = "le_secret :)".to_string();
    let config_reader = ConfigReader::default();
    let mut config_path = get_test_data_path(file!());
    config_path.push("config.yaml");
    let config = config_path.to_str().unwrap();
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
