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
    /// * `packageData` - configuration package data bytes.
    /// * `targetPath` - path into which the extracted files from the package will be stored.
    ///
    /// # Returns
    ///
    /// * __Ok__(`()`) - successfully extracted `packageData` into the `targetPath`.
    /// * __Err__(`Error`) - error indicating what went wrong.
    fn extract(packageData: Vec<u8>, targetPath: &str) -> Result<(), Error>;
}
