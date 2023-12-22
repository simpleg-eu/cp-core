/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */
use crate::error::Error;
#[macro_export]
macro_rules! ok_or_return_error {
    ($expression: expr, $error_kind: expr, $error_message: expr) => {
        match $expression {
            Ok(value) => value,
            Err(error) => {
                return Err(Error::new(
                    $error_kind,
                    format!("{}{}", $error_message, error),
                ))
            }
        }
    };
}

#[macro_export]
macro_rules! some_or_return_error {
    ($expression: expr, $error_kind: expr, $error_message: expr) => {
        match $expression {
            Some(value) => value,
            None => return Err(Error::new($error_kind, $error_message)),
        }
    };
}
