/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */
use std::env;
use std::path::{Path, PathBuf};

pub fn get_test_data_path(test_file_name: &str) -> PathBuf {
    let test_file_path = Path::new(test_file_name);
    let test_file_name_without_extension = test_file_path
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    let mut path = env::current_dir().unwrap();
    path.push("test_data");
    path.push(test_file_name_without_extension);

    path
}
