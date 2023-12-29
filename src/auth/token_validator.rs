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
    ///
    /// Validates the specified token.
    ///
    /// # Arguments
    ///
    /// * `token` - Token to be validated.
    ///
    /// # Returns
    ///
    /// * `Ok` - Token.
    /// * `Err` - Error if the validation has failed.
    /// Having the error kind valued as `INVALID_TOKEN` if the token is invalid.
    fn validate(&self, token: &str) -> Result<Arc<dyn Token + Send + Sync>, Error>;
}
