/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */
use crate::error::Error;
#[macro_export]
macro_rules! ok_or_return_error {
    ($expression: expr, $error_kind: ident, $error_message: ident) => {
        match $expression {
            Ok(value) => value,
            Err(error) => return Error::new($error_kind, format!("{}{}", $error_message, error)),
        }
    };
}
