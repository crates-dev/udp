//! udp
//!
//! A lightweight and efficient Rust library for
//! building UDP servers with request-response handling.

mod attribute;
mod common;
mod config;
mod context;
mod error;
mod hook;
mod panic;
mod request;
mod response;
mod server;
mod socket;
mod utils;

pub use {
    attribute::*, common::*, config::*, context::*, error::*, hook::*, panic::*, request::*,
    response::*, server::*, socket::*, utils::*,
};

pub use tokio;

use std::{
    any::Any,
    collections::HashMap,
    error::Error as StdError,
    fmt::{self, Display},
    future::Future,
    net::{IpAddr, SocketAddr},
    pin::Pin,
    sync::Arc,
};

use tokio::{
    net::UdpSocket,
    sync::{
        RwLock, RwLockReadGuard, RwLockWriteGuard,
        watch::{Receiver, Sender, channel},
    },
    task::JoinHandle,
};
