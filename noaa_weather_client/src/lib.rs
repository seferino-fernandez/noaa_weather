//! NOAA Weather API client library
//!
//! This crate provides a complete client for the NOAA Weather API,
//! including all endpoints for forecasts, alerts, observations, and more.

pub mod apis;
pub mod models;
pub mod utils;

pub use apis::configuration::Configuration;
