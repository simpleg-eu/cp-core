/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use serde_yaml::Value;

use cp_core::config_reader::ConfigReader;
use cp_core::error::Error;

use crate::test_base::get_test_data_path;

mod test_base;

#[test]
fn get_config_existing_config_file_returns() {
    let mut test_data_path = get_test_data_path(file!());
    test_data_path.push("config.yaml");
    let config_reader: ConfigReader = ConfigReader::default();

    let result: Result<Value, Error> = config_reader.read(test_data_path);
    let root_value = result.unwrap();
    let example_container = root_value.get("Config").unwrap().get("Example").unwrap();
    let inner_value: i64 = example_container
        .get("InnerValue")
        .unwrap()
        .as_i64()
        .unwrap();
    let inner_bool: bool = example_container
        .get("InnerBool")
        .unwrap()
        .as_bool()
        .unwrap();
    let inner_string: &str = example_container
        .get("InnerString")
        .unwrap()
        .as_str()
        .unwrap();

    assert_eq!(1234, inner_value);
    assert!(inner_bool);
    assert_eq!("yes", inner_string);
}
