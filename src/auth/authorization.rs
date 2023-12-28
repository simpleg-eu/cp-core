/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::sync::Arc;

use axum::http::HeaderMap;

use crate::auth::error_kind::INVALID_HEADERS;
use crate::auth::token_validator::TokenValidator;
use crate::error::Error;
use crate::{ok_or_return_error, some_or_return_error};

pub const AUTHORIZATION_HEADER: &str = "Authorization";

pub struct Authorization {
    token_validator: Arc<dyn TokenValidator + Send + Sync>,
}

impl Authorization {
    pub fn new(token_validator: Arc<dyn TokenValidator + Send + Sync>) -> Self {
        Self { token_validator }
    }

    ///
    /// Checks if there is an `Authorization` header which contains a valid token.
    ///
    /// # Arguments
    ///
    /// * `headers` - Map of the headers of the request.
    ///
    /// # Returns
    ///
    /// * `Ok` - If the headers contain a valid authorization header with a valid token.
    /// * `Err` - If the validation failed.
    pub async fn validate(&self, headers: HeaderMap) -> Result<(), Error> {
        let authorization_value = some_or_return_error!(
            headers.get(AUTHORIZATION_HEADER),
            INVALID_HEADERS,
            "'Authorization' header is missing"
        );

        let authorization = ok_or_return_error!(
            authorization_value.to_str(),
            INVALID_HEADERS,
            "could not read 'Authorization' header as string"
        );

        // skip 'Bearer '
        if authorization.len() < 7 {
            return Err(Error::new(
                INVALID_HEADERS,
                "'Authorization' header value is invalid",
            ));
        }

        let token = &authorization[7..];

        self.token_validator.validate(token)?;

        Ok(())
    }
}

#[cfg(test)]
pub mod tests {
    use std::sync::Arc;

    use axum::http::{HeaderMap, HeaderValue};
    use mockall::predicate::eq;

    use crate::auth::authorization::{Authorization, AUTHORIZATION_HEADER};
    use crate::auth::token::MockToken;
    use crate::auth::token_validator::MockTokenValidator;

    #[tokio::test]
    pub async fn validate_extracts_token_from_header() {
        let mut headers = HeaderMap::new();
        let expected_token = "1234abcd";
        let mut token_validator_mock = MockTokenValidator::new();
        token_validator_mock
            .expect_validate()
            .with(eq(expected_token))
            .times(1)
            .returning(|_| {
                let token_mock = MockToken::new();

                Ok(Arc::new(token_mock))
            });
        let authorization_header =
            HeaderValue::from_str(format!("Bearer {}", expected_token).as_str())
                .expect("expected 'authorization' HeaderValue");
        headers.append(AUTHORIZATION_HEADER, authorization_header);
        let authorization = Authorization::new(Arc::new(token_validator_mock));

        let result = authorization.validate(headers).await;

        assert!(result.is_ok());
    }
}
