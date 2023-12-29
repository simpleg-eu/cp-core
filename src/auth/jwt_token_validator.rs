/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::collections::HashMap;
use std::str::FromStr;
use std::sync::Arc;

use jsonwebtoken::jwk::{AlgorithmParameters, JwkSet};
use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use serde_json::Value;

use crate::auth::error_kind::{INVALID_TOKEN, JWKS_RETRIEVAL_FAILURE, MALFORMED_TOKEN};
use crate::auth::jwt_token::JwtToken;
use crate::auth::token::Token;
use crate::auth::token_validator::TokenValidator;
use crate::error::Error;
use crate::{ok_or_return_error, some_or_return_error};

pub struct JwtTokenValidator {
    jwk_set: JwkSet,
    issuers: Vec<String>,
    audience: Vec<String>,
}

impl JwtTokenValidator {
    pub fn new(jwk_set: JwkSet, issuers: Vec<String>, audience: Vec<String>) -> Self {
        Self {
            jwk_set,
            issuers,
            audience,
        }
    }
}

impl TokenValidator for JwtTokenValidator {
    fn validate(&self, token: &str) -> Result<Arc<dyn Token + Send + Sync>, Error> {
        let header = ok_or_return_error!(
            decode_header(token),
            MALFORMED_TOKEN,
            "failed to decode token's header: "
        );

        let kid =
            some_or_return_error!(header.kid, MALFORMED_TOKEN, "'kid' is missing from header");

        let jwk = some_or_return_error!(
            self.jwk_set.find(&kid),
            MALFORMED_TOKEN,
            "could not find 'kid' within 'jwk_set'"
        );

        let rsa = match jwk.algorithm {
            AlgorithmParameters::RSA(ref rsa) => rsa,
            _ => {
                return Err(Error::new(
                    MALFORMED_TOKEN,
                    format!(
                        "got an unexpected algorithm for the 'jwk': {:?}",
                        jwk.algorithm
                    ),
                ));
            }
        };

        let decoding_key = ok_or_return_error!(
            DecodingKey::from_rsa_components(&rsa.n, &rsa.e),
            MALFORMED_TOKEN,
            "failed to get decoding key: "
        );

        let key_algorithm = some_or_return_error!(
            jwk.common.key_algorithm,
            MALFORMED_TOKEN,
            "'jwk' is missing the 'key_algorithm'"
        );

        let algorithm = ok_or_return_error!(
            Algorithm::from_str(key_algorithm.to_string().as_str()),
            MALFORMED_TOKEN,
            "failed to get algorithm for 'key_algorithm': "
        );

        let mut validation = Validation::new(algorithm);
        validation.validate_exp = true;
        validation.set_issuer(self.issuers.as_slice());
        validation.set_audience(self.audience.as_slice());

        let decoded_token = ok_or_return_error!(
            decode::<HashMap<String, Value>>(token, &decoding_key, &validation),
            INVALID_TOKEN,
            "failed to validate token: "
        );

        Ok(Arc::new(JwtToken::new(decoded_token)))
    }
}

/// Tries to get a `JwkSet` by reaching to the specified `jwks_uri`.
///
/// # Arguments
///
/// * `jwks_uri` - URI to be reached for retrieving the `JwkSet`.
pub async fn try_get_jwks(jwks_uri: &str) -> Result<JwkSet, Error> {
    let response = ok_or_return_error!(
        reqwest::get(jwks_uri).await,
        JWKS_RETRIEVAL_FAILURE,
        "invalid response obtained getting the 'JwkSet'"
    );

    let jwks = ok_or_return_error!(
        response.json::<JwkSet>().await,
        JWKS_RETRIEVAL_FAILURE,
        "response cannot be deserialized as a 'JwkSet'"
    );

    Ok(jwks)
}

#[cfg(test)]
pub mod tests {
    use std::path::PathBuf;

    use crate::auth::error_kind::{INVALID_TOKEN, MALFORMED_TOKEN};
    use crate::auth::jwt_token_validator::{try_get_jwks, JwtTokenValidator};
    use crate::auth::token_validator::TokenValidator;
    use crate::config_reader::ConfigReader;
    use crate::secrets::get_secrets_manager;
    use crate::test_base::get_unit_test_data_path;

    #[tokio::test]
    pub async fn validate_expired_jwt_token_returns_error() {
        let config_reader = ConfigReader::default();
        let config_path = get_config_file();
        let config_value = config_reader
            .read(config_path)
            .expect("expected config reader");
        let token_validator_config = config_value
            .get("TokenValidator")
            .expect("expected 'TokenValidator'");
        let jwks_uri = token_validator_config
            .get("JwksUri")
            .expect("expected 'JwksUri'")
            .as_str()
            .expect("expected 'JwksUri' as string");
        let jwks = try_get_jwks(jwks_uri).await.expect("expected 'JwkSet'");
        let issuers: Vec<String> = token_validator_config
            .get("Issuers")
            .expect("expected 'Issuers'")
            .as_sequence()
            .expect("expected 'Issuers' as sequence")
            .iter()
            .map(|v| {
                v.as_str()
                    .expect("expected 'Issuers' value as string")
                    .to_string()
            })
            .collect();
        let audience: Vec<String> = token_validator_config
            .get("Audience")
            .expect("expected 'Audience'")
            .as_sequence()
            .expect("expected 'Audience' as sequence")
            .iter()
            .map(|v| {
                v.as_str()
                    .expect("expected 'Audience' value as string")
                    .to_string()
            })
            .collect();
        let secrets_manager = get_secrets_manager().expect("failed to get secrets manager");
        let expired_token = secrets_manager
            .get_secret(
                config_value
                    .get("ExpiredTokenSecret")
                    .expect("expected 'ExpiredTokenSecret'")
                    .as_str()
                    .expect("expected 'ExpiredTokenSecret' as string"),
            )
            .expect("failed to get expired token from secrets manager");
        let token_validator = JwtTokenValidator::new(jwks, issuers, audience);

        let error = token_validator
            .validate(expired_token.as_str())
            .expect_err("expected validation failure due to token having to be expired");

        assert_eq!(INVALID_TOKEN, error.error_kind());
        assert!(error.message().contains("failed to validate token"));
    }

    #[tokio::test]
    pub async fn try_get_jwks_succeeds_for_example_uri() {
        let config_reader = ConfigReader::default();
        let config_path = get_config_file();
        let config_value = config_reader
            .read(config_path)
            .expect("expected config reader");
        let token_validator_config = config_value
            .get("TokenValidator")
            .expect("expected 'TokenValidator'");
        let jwks_uri = token_validator_config
            .get("JwksUri")
            .expect("expected 'JwksUri'")
            .as_str()
            .expect("expected 'JwksUri' as string");

        let result = try_get_jwks(jwks_uri).await;

        assert!(result.is_ok());
    }

    fn get_config_file() -> PathBuf {
        let mut config_path = get_unit_test_data_path(file!());
        config_path.push("config.yaml");

        config_path
    }
}
