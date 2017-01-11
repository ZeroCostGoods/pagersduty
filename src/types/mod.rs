pub mod reference;
pub mod abilities;

// Modules that use serde-codegen
pub mod contact_methods { include!(concat!(env!("OUT_DIR"), "/types/contact_methods.rs")); }
pub mod notification_rules { include!(concat!(env!("OUT_DIR"), "/types/notification_rules.rs")); }
pub mod teams { include!(concat!(env!("OUT_DIR"), "/types/teams.rs")); }
pub mod users { include!(concat!(env!("OUT_DIR"), "/types/users.rs")); }
