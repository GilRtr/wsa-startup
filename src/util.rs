//! This module holds functions that allow one to really easily start up WSA

use crate::{Result, Wsa, WsaInitializer};

/// Initialize WSA with default zeroed options and version 2.2
/// # Errors
/// This function will return a [`WsaError`] when `WSAStartup` fails
pub fn try_wsa_startup() -> Result<Wsa> {
    WsaInitializer::default().init()
}

/// Initialize WSA with default zeroed options and version 2.2
/// # Panics
/// This may panic if `WSAStartup` fails
pub fn wsa_startup() -> Wsa {
    try_wsa_startup().unwrap()
}
