#![allow(clippy::new_without_default)]

pub mod firestore;
pub mod rdb;
mod result;
pub use firebase_client_admin_auth as admin_auth;
pub use firebase_client_auth as auth;

pub use result::*;
