//! This library provides access to various PagerDuty APIs.

// #![warn(missing_docs)]

extern crate hyper;
extern crate hyper_native_tls;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;

pub mod errors;
pub mod events;
pub mod rest;

