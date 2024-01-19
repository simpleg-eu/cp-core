/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::fmt::{Display, Formatter};

use crate::error_kind::{SERIALIZATION_FAILURE, REQUEST_FAILURE};

#[derive(Debug, Clone, PartialOrd, PartialEq)]
pub struct Error {
    error_kind: String,
    message: String,
}

impl Error {
    pub fn new(error_kind: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            error_kind: error_kind.into(),
            message: message.into(),
        }
    }

    pub fn error_kind(&self) -> &str {
        self.error_kind.as_str()
    }

    pub fn message(&self) -> &str {
        self.message.as_str()
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.error_kind, self.message)
    }
}

impl From<serde_yaml::Error> for Error {
    fn from(value: serde_yaml::Error) -> Self {
        Self {
            error_kind: SERIALIZATION_FAILURE.to_string(),
            message: value.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(value: serde_json::Error) -> Self {
        Self {
            error_kind: SERIALIZATION_FAILURE.to_string(),
            message: value.to_string(),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self {
            error_kind: value.kind().to_string(),
            message: value.to_string(),
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(value: reqwest::Error) -> Self {
        Self {
            error_kind: REQUEST_FAILURE.to_string(),
            message: value.to_string()
        }
    }
}
