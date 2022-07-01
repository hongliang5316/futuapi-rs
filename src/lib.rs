#![allow(dead_code)]
#![allow(clippy::from_over_into)]
#![allow(clippy::too_many_arguments)]

#[macro_use]
extern crate lazy_static;

include!(concat!(env!("OUT_DIR"), "/rust_protobuf_protos/mod.rs"));

pub mod action;
pub mod client;
pub mod connection;
pub use connection::Connection;
pub mod frame;
pub use frame::serial_no;
pub use frame::Frame;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
