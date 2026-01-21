//! udp
//!
//! A lightweight and efficient Rust library for
//! building UDP servers with request-response handling.

mod common;
mod config;
mod context;
mod handler;
mod request;
mod response;
mod server;
mod socket;
mod utils;

pub use {config::*, context::*, request::*, response::*, server::*, socket::*, utils::*};

pub use tokio;

use {common::*, handler::*};

use std::{
    any::Any,
    collections::HashMap,
    error::Error as StdError,
    fmt::{self, Display},
    future::Future,
    net::{IpAddr, Ipv4Addr, SocketAddr, SocketAddrV4},
    panic::set_hook,
    pin::Pin,
    sync::Arc,
};

use tokio::{
    net::UdpSocket,
    sync::{MutexGuard, RwLock, RwLockReadGuard, RwLockWriteGuard},
};
