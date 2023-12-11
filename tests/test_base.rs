/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */
use std::path::{Path, PathBuf};

pub fn get_test_data_path(test_file_name: &str) -> PathBuf {
    let test_data_path_string = test_file_name
        .replace("tests/", "test_data/")
        .replace(".rs", "");
    let test_data_path = Path::new(test_data_path_string.as_str());

    test_data_path.to_path_buf()
}
