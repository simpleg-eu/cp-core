use std::collections::HashMap;

use serde::de::DeserializeOwned;
use serde_yaml::Value;

use crate::{
    config::getter::Getter, config_reader::ConfigReader, error::Error, error_kind::NOT_FOUND,
};

const KEY_SPLIT: &str = ":";

pub struct FileGetter {
    target_path: String,
    config_reader: ConfigReader,
    cache: HashMap<String, Value>,
}

impl FileGetter {
    fn new(target_path: String) -> Self {
        Self {
            target_path,
            config_reader: ConfigReader::default(),
            cache: HashMap::new(),
        }
    }

    fn inner_value(value: &Value, keys: &[&str], index: usize) -> Result<Value, Error> {
        let current_key = keys[index];

        let current_value = match value.get(current_key) {
            Some(current_value) => current_value,
            None => {
                return Err(Error::new(
                    NOT_FOUND,
                    format!("could not find key '{}'", current_key),
                ))
            }
        };

        if index < keys.len() - 1 {
            return FileGetter::inner_value(current_value, keys, index + 1);
        }

        return Ok(current_value.clone());
    }

    fn insert_cache(&mut self, file_path: &str, value: Value) -> Result<&Value, Error> {
        self.cache.insert(file_path.to_string(), value);

        match self.cache.get(file_path) {
            Some(value) => Ok(value),
            None => Err(Error::new(
                NOT_FOUND,
                format!(
                    "could not find file path '{}' within cache after insertion",
                    &file_path
                ),
            )),
        }
    }
}

impl Getter for FileGetter {
    fn get<T: DeserializeOwned>(&mut self, file_path: &str, key: &str) -> Result<T, Error> {
        let value = match self.cache.get(file_path) {
            Some(value) => value,
            None => {
                let complete_path = format!("{}/{}", &self.target_path, &file_path);

                match self.config_reader.read(complete_path.into()) {
                    Ok(value) => self.insert_cache(file_path, value)?,
                    Err(error) => return Err(error),
                }
            }
        };

        let keys: Vec<&str> = key.split(KEY_SPLIT).collect();

        let value = FileGetter::inner_value(value, keys.as_slice(), 0usize)?;

        let config_entry = match serde_yaml::from_value::<T>(value) {
            Ok(config_entry) => config_entry,
            Err(error) => return Err(error.into()),
        };

        return Ok(config_entry);
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        config::{file_getter::FileGetter, getter::Getter},
        error_kind::NOT_FOUND,
        test_base::get_unit_test_data_path,
    };

    #[tokio::test]
    pub async fn get_returns_expected_value() {
        let expected_value: i64 = 5;
        let mut getter = get_getter();

        let result = getter
            .get::<i64>("application.yaml", "Example:Inner:Value")
            .expect("expected 'i64' got an error instead");

        assert_eq!(expected_value, result);
    }

    #[tokio::test]
    pub async fn get_root_key_returns_expected_value() {
        let expected_value = "yes";
        let mut getter = get_getter();

        let result = getter
            .get::<String>("application.yaml", "Root")
            .expect("expected 'String' got an error instead");

        assert_eq!(expected_value, result);
    }

    #[tokio::test]
    pub async fn get_inner_key_returns_expected_value() {
        let expected_value = true;
        let mut getter = get_getter();

        let result = getter
            .get::<bool>("application.yaml", "Example:Yeah")
            .unwrap();

        assert_eq!(expected_value, result);
    }

    #[tokio::test]
    pub async fn get_not_existing_key_returns_error() {
        let mut getter = get_getter();
        let result = getter.get::<bool>("application.yaml", "Lmao");

        assert!(result.is_err());
        assert_eq!(NOT_FOUND, result.unwrap_err().error_kind());
    }

    #[tokio::test]
    pub async fn get_not_existing_file_returns_error() {
        let mut getter = get_getter();
        let result = getter.get::<bool>("loooool.yaml", "yes");

        assert!(result.is_err());
        assert_eq!(NOT_FOUND, result.unwrap_err().error_kind());
    }

    fn get_getter() -> FileGetter {
        let target_path = get_unit_test_data_path(file!());
        let getter = FileGetter::new(target_path.to_str().unwrap().to_string());

        getter
    }
}
