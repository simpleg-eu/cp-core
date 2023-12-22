/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use crate::auth::token::Token;
use crate::error::Error;
use std::sync::Arc;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait TokenValidator {
    fn validate(&self, token: &str) -> Result<Arc<dyn Token + Send + Sync>, Error>;
}
