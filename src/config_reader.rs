/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::path::PathBuf;

use serde_yaml::Value;

use crate::error::Error;

#[derive(Default)]
pub struct ConfigReader {}

impl ConfigReader {
    pub fn read(&self, config_file_path: PathBuf) -> Result<Value, Error> {
        let result = std::fs::read_to_string(config_file_path);

        match result {
            Ok(yaml) => match serde_yaml::from_str(yaml.as_str()) {
                Ok(value) => Ok(value),
                Err(error) => Err(error.into()),
            },
            Err(error) => Err(error.into()),
        }
    }
}
