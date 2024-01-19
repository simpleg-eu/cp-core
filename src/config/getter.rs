/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2024.
 */

use serde::de::DeserializeOwned;

use crate::error::Error;

/// `Getter` offers the ability to get configuration values from previously
/// extracted packages.
pub trait Getter {
    /// `Get` provides the configuration value for the specified key combination.
    ///
    /// # Arguments
    ///
    /// * `file_path` - string indicating the file path relative to
    /// the configuration's extractor target path.
    /// * `key` - index that supports nesting by using ':', i.e. `Root:Parent:Child:ExampleString`.
    ///
    /// # Returns
    ///
    /// * __Ok__(`T`) - the configuration value with the specified type.
    /// * __Err__(`Error`) - error indicating what went wrong.
    fn get<T: DeserializeOwned>(&mut self, file_path: &str, key: &str) -> Result<T, Error>;
}
