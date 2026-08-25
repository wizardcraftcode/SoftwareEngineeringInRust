use std::error::Error;
use std::fmt;
use std::fs;
use std::num::ParseIntError;
use thiserror::Error;

// because we have two errors that result from ParseIntError, we can't use the from annotation
#[derive(Error,Debug)]
pub enum ConfigError {
    #[error("failed to read config file: {0}")]
    FileError(#[from]std::io::Error),

    #[error("invalid digit in port number")]
    InvalidPort(#[source] ParseIntError),

    #[error("invalid timeout")]
    InvalidTimeout (#[source] ParseIntError),

    #[error("missing required field: {0}")]
    MissingField(String),
}

#[derive(Debug, PartialEq, Eq)]
pub struct AppConfig {
    pub port: u16,
    pub timeout: u64,
}

pub fn parse_config_content(content: &str) -> Result<AppConfig, ConfigError> {
    let mut port = None;
    let mut timeout = None;

    for  line in content.lines() {
        if let Some((key, val)) = line.split_once('=') {
            let val = val.trim();
            match key.trim() {
                "PORT" => {
                    let p =
                        val.parse()
                            .map_err(|source| ConfigError::InvalidPort (source))?;
                    port = Some(p);
                }
                "TIMEOUT" => {
                    let t = val.parse().map_err(|source| ConfigError::InvalidTimeout (
                        source,
                    ))?;
                    timeout = Some(t);
                }
                _ => {}
            }
        }
    }

    let port = port.ok_or(ConfigError::MissingField("PORT".to_string()))?;
    let timeout = timeout.ok_or(ConfigError::MissingField("TIMEOUT".to_string()))?;
    Ok(AppConfig { port, timeout })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error;
    use std::num::ParseIntError;

    #[test]
    fn invalid_port_returns_port_variant_with_line_number() {
        let content = "PORT=bad_port\nTIMEOUT=30";
        let err = parse_config_content(content).unwrap_err();

        assert!(matches!(err, ConfigError::InvalidPort(_)));
        assert!(err.source().unwrap().is::<ParseIntError>());
    }

    #[test]
    fn invalid_timeout_returns_timeout_variant_with_line_number() {
        let content = "PORT=8080\nTIMEOUT=bad_timeout";
        let err = parse_config_content(content).unwrap_err();

        assert_eq!(err.to_string(), "invalid timeout");
        assert!(err.source().unwrap().is::<ParseIntError>());
    }
}