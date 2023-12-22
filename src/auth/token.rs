/*
 * Copyright (c) Gabriel Amihalachioaie, SimpleG 2023.
 */

use std::fmt::Debug;

#[cfg(test)]
use mockall::automock;

#[cfg_attr(test, automock)]
pub trait Token: Debug {}
