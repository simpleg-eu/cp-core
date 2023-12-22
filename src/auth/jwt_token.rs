/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use crate::auth::token::Token;
use jsonwebtoken::TokenData;
use serde_json::Value;
use std::collections::HashMap;
use std::fmt::{Debug, Formatter};

pub struct JwtToken {
    token_data: TokenData<HashMap<String, Value>>,
}

impl JwtToken {
    pub fn new(token_data: TokenData<HashMap<String, Value>>) -> Self {
        Self { token_data }
    }
}

impl Debug for JwtToken {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{:?}", self.token_data).as_str())
    }
}

impl Token for JwtToken {}
