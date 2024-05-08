use crate::prelude::*;

pub fn create_directory(path: &str) -> Result<&std::path::Path> {
    let directory = std::path::Path::new(path);

    if !directory.exists() {
        std::fs::create_dir_all(directory).map_err(Error::io)?;
    }

    Ok(directory)
}

pub fn create_or_open_file(file_name: &str, directory: &std::path::Path) -> Result<std::fs::File> {
    let path = std::path::Path::new(file_name);
    let file = std::fs::File::options()
        .append(true)
        .create(true)
        .open(directory.join(path))
        .map_err(Error::io)?;
    Ok(file)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::Builder;

    #[tokio::test]
    async fn create_directory_when_called_with_valid_path_should_create_directory_and_return_path_object(
    ) {
        let temp_dir = Builder::new()
            .prefix("test_dir")
            .tempdir()
            .expect("Cant create temp dir.");
        let path = temp_dir.path().join("test_sub_dir");

        let result = create_directory(path.to_str().unwrap());

        assert_eq!(path, result.unwrap());
    }

    #[tokio::test]
    async fn create_directory_when_called_with_invalid_path_should_err_with_io_error() {
        let invalid_path = "/invalid/path";

        let result = create_directory(invalid_path);

        assert_eq!(
            result.unwrap_err(),
            Error::io("Permission denied (os error 13)")
        );
    }

    #[tokio::test]
    async fn create_or_open_file_when_called_with_valid_path_should_create_or_open_file_and_return_file_object(
    ) {
        let temp_dir = Builder::new()
            .prefix("test_dir")
            .tempdir()
            .expect("Cant create temp dir.");
        let directory = temp_dir.path();
        let file_name = "test_file.txt";

        let result = create_or_open_file(file_name, directory);

        assert!(result.is_ok());
        assert!(result.unwrap().metadata().is_ok());
    }

    #[tokio::test]
    async fn create_or_open_file_when_called_with_existing_file_should_return_existing_file_object()
    {
        let temp_dir = Builder::new()
            .prefix("test_dir")
            .tempdir()
            .expect("Cant create temp dir.");
        let directory = temp_dir.path();
        let file_name = "test_file.txt";

        std::fs::File::create(directory.join(file_name)).unwrap();

        let result = create_or_open_file(file_name, directory);

        assert!(result.is_ok());
        assert!(result.unwrap().metadata().is_ok());
    }

    #[tokio::test]
    async fn create_or_open_file_when_called_with_invalid_path_should_err_with_io_error() {
        let directory = std::path::Path::new("/invalid/path");
        let file_name = "test_file.txt";

        let result = create_or_open_file(file_name, directory);

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err(),
            Error::io("No such file or directory (os error 2)")
        );
    }
}
