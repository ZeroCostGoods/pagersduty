//! This library provides access to the Pagerduty v2 API.

extern crate hyper;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod client;
pub mod errors;
pub mod events;
pub mod types;

