/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2024.
 */

use crate::error::Error;

/// `Extractor` provides the ability to extract a previously downloaded package data.
pub trait Extractor {
    /// `Extract` extracts the configuration package's content into the `targetPath`.
    ///
    /// # Arguments
    ///
    /// * `package_data` - configuration package data bytes.
    /// * `target_path` - path into which the extracted files from the package will be stored.
    ///
    /// # Returns
    ///
    /// * __Ok__(`()`) - successfully extracted `packageData` into the `targetPath`.
    /// * __Err__(`Error`) - error indicating what went wrong.
    fn extract(&self, package_data: Vec<u8>, target_path: &str) -> Result<(), Error>;
}
