use crate::{config::extractor::Extractor, error::Error};

#[derive(Default)]
pub struct ZipExtractor {}

impl Extractor for ZipExtractor {
    fn extract(&self, package_data: Vec<u8>, target_path: &str) -> Result<(), Error> {
        let reader = std::io::Cursor::new(package_data);
        let mut archive = match zip::ZipArchive::new(reader) {
            Ok(archive) => archive,
            Err(error) => return Err(error.into())
        };

        match archive.extract(target_path) {
            Ok(_) => (),
            Err(error) => return Err(error.into())
        }

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{config::{zip_extractor::ZipExtractor, extractor::Extractor}, test_base::get_unit_test_data_path};

    #[tokio::test]
    pub async fn extract_zip_file_correctly() {
        let mut path = get_unit_test_data_path(file!());
        path.push("dummy.zip");
        let extractor = ZipExtractor::default();
        let target_path = uuid::Uuid::new_v4().to_string();
        let file = tokio::fs::read(path).await.expect("expected file");
        
        let result = extractor.extract(file, target_path.as_str());
        let target_path_exists = does_path_exist(&target_path).await;
        let executable_exists = does_path_exist(&format!("{}/cp-config", &target_path)).await;
        let config_dir_exists = does_path_exist(&format!("{}/config", &target_path)).await;
        let config_exists = does_path_exist(&format!("{}/config/config.yaml", &target_path)).await;
        let log_config_exists = does_path_exist(&format!("{}/config/log4rs.yaml", &target_path)).await;
        let subfolder_exists = does_path_exist(&format!("{}/config/subfolder", &target_path)).await;
        let another_exists = does_path_exist(&format!("{}/config/subfolder/another.yaml", &target_path)).await;
        
        let _ = std::fs::remove_dir_all(target_path);
        assert!(result.is_ok());
        assert!(target_path_exists);
        assert!(executable_exists);
        assert!(config_dir_exists);
        assert!(config_exists);
        assert!(log_config_exists);
        assert!(subfolder_exists);
        assert!(another_exists);
    }

    async fn does_path_exist(path: &str) -> bool {
        return tokio::fs::metadata(path).await.is_ok();
    }
}
