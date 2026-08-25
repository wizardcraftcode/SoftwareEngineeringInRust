// use std::error::Error;
use std::fs;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("failed to read configuration file: {0:?}")]
    FileError(#[from]std::io::Error),
}

pub fn load_config(path: &str) -> Result<String, ConfigError> {
    let result = fs::read_to_string(path);
    return match result {
        Ok(contents) => Ok(contents),
        Err(err) => Err(ConfigError::FileError(err)),
    }
}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::io::{Error as IoError, ErrorKind};
    use super::*;

    #[test]
    fn format_error_displays_correctly() {
        let io_err = IoError::new(ErrorKind::NotFound, "file not found");
        let err = ConfigError::FileError(io_err);

        // .to_string() invokes the Display trait
        assert_eq!(
            err.to_string(),
            "failed to read configuration file: file not found"
        );
    }

    #[test]
    fn test_missing_file_returns_io_error_with_source() {
        let err = load_config("non_existent_file.toml").unwrap_err();

        // 1. Idiomatic variant check
        assert!(matches!(err, ConfigError::FileError(_)));

        // 2. Verify source() chaining works
        let source = err.source().expect("ConfigError::Io must provide a source");
        assert!(source.is::<std::io::Error>());
    }

    #[test]
    fn config_error_io_exposes_underlying_source() {
        let io_err = IoError::new(ErrorKind::PermissionDenied, "access denied");
        let err = ConfigError::FileError(io_err);

        // Call .source() explicitly as a trait method from std::error::Error
        let source = err.source().expect("ConfigError::Io must return a source error");

        // Verify the source can be downcast to std::io::Error
        assert!(source.is::<std::io::Error>());

        // Verify details on the underlying source error
        let downcast_io_err = source.downcast_ref::<std::io::Error>().unwrap();
        assert_eq!(downcast_io_err.kind(), ErrorKind::PermissionDenied);
    }

}