/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use crate::error::Error;

pub trait SecretsManager {
    fn get_secret(&self, secret_id: &str) -> Result<String, Error>;
}
