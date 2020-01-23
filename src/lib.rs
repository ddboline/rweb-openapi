//! Openapi provides structures and support for serializing and deserializing [openapi](https://github.com/OAI/OpenAPI-Specification) specifications
//!
//! # Examples
//!
//! Typical use deserialing an existing to a persisted spec to rust form or
//! visa versa
//!
//! The hyper client should be configured with tls.
//!
//! ```no_run
//! extern crate openapi;
//!
//! fn main() {
//!   match openapi::from_path("path/to/openapi.yaml") {
//!     Ok(spec) => println!("spec: {:?}", spec),
//!     Err(err) => println!("error: {}", err)
//!   }
//! }
//! ```
//!
//! # Errors
//!
//! Operations typically result in a `openapi::Result` Type which is an alias
//! for Rust's
//! built-in Result with the Err Type fixed to the
//! [openapi::errors::Error](errors/struct.Error.html) enum type. These are
//! provided
//! using [error_chain](https://github.com/brson/error-chain) crate so their
//! shape and behavior should be consistent and familiar to existing
//! error_chain users.
//!
pub use error::Error;
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, result::Result as StdResult};

pub mod error;
pub mod v3_0;

pub type Str = Cow<'static, str>;

const MINIMUM_OPENAPI30_VERSION: &str = ">= 3.0";

pub type Result<T> = StdResult<T, Error>;

/// Supported versions of the OpenApi.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(untagged)]
pub enum OpenApi {
    /// Version 3.0.1 of the OpenApi specification.
    ///
    /// Refer to the official
    /// [specification](https://github.com/OAI/OpenAPI-Specification/blob/0dd79f6/versions/3.0.1.md)
    /// for more information.
    #[allow(non_camel_case_types)]
    V3_0(v3_0::Spec),
}
