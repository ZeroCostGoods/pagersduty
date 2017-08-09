//! This library provides access to various PagerDuty APIs.

// #![warn(missing_docs)]

extern crate reqwest;
extern crate serde;
#[macro_use] extern crate serde_derive;

// We only use the macros in tests currently.
// This shuts up the warnings.
#[cfg(not(test))]
extern crate serde_json;
#[cfg(test)]
#[macro_use]
extern crate serde_json;

pub mod errors;
pub mod events;
pub mod rest;

