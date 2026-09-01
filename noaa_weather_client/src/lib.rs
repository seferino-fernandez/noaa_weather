#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod apis;
pub mod client;
pub mod models;
pub mod utils;

pub use apis::{BinaryPayload, Error, ProtocolError, RedirectReason, ResponseContent};
pub use client::{BuildError, Client, ClientBuilder, RetryPolicy};
