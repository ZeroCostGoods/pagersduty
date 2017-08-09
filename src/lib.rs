//! This library provides access to various PagerDuty APIs.

// #![warn(missing_docs)]

extern crate reqwest;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;

pub mod errors;
pub mod events;
pub mod rest;

