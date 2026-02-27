#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod apis;
pub mod models;
pub mod utils;

pub use apis::configuration::Configuration;
