//! This library provides access to the Pagerduty v2 API.

// #![warn(missing_docs)]

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod client;
pub mod errors;
pub mod events;
pub mod types;

