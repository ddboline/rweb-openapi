//! Openapi provides structures and support for serializing and deserializing [openapi](https://github.com/OAI/OpenAPI-Specification) specifications
//!
//! # Examples
//!
//! Typical use deserialing an existing to a persisted spec to rust form or
//! visa versa
//!
//! The hyper client should be configured with tls.
//!
use std::borrow::Cow;

pub mod v3_0;

pub type Str = Cow<'static, str>;
