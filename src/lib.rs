//! This crate allows you to initialize WSA

#![cfg(windows)]
#![warn(clippy::pedantic, clippy::nursery, clippy::cargo)]

pub mod util;

use std::{
    error::Error,
    fmt::{Display, Formatter, Result as FmtResult},
};
use winapi::{
    shared::minwindef::MAKEWORD as make_version,
    um::winsock2::{self as win, WSADATA},
};

/// Convenience type alias for a result that errs on [`WsaError`]
pub type Result<T, E = WsaError> = std::result::Result<T, E>;

/// An Error returned from `WSAStartup`
#[derive(Debug)]
pub enum WsaError {
    SystemNotReady,
    VersionNotSupported,
    OperationInProgress,
    TasksLimitReached,
    InvalidData,
    UnknownError,
}

use WsaError::{
    InvalidData, OperationInProgress, SystemNotReady, TasksLimitReached, UnknownError,
    VersionNotSupported,
};

impl Error for WsaError {}

impl Display for WsaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        const ERR_CODES: &str =
            "https://docs.microsoft.com/en-us/windows/win32/winsock/windows-sockets-error-codes-2";
        const GENERAL: &str =
            "https://docs.microsoft.com/en-us/windows/win32/api/winsock/nf-winsock-wsastartup";

        match self {
            UnknownError => write!(f, "Some unknown error has occurred, it's time to panic.\nsee \"{}\" ", GENERAL)?,
            SystemNotReady => write!(f, "The underlying network subsystem is not ready for network communication.\nsee \"{}/#WSASYSNOTREADY\" ", ERR_CODES)?,
            VersionNotSupported => write!(f, "The version of Windows Sockets support requested is not provided by this particular Windows Sockets implementation.\nsee \"{}/#WSAVERNOTSUPPORTED\" ", ERR_CODES)?,
            OperationInProgress => write!(f, "A blocking Windows Sockets 1.1 operation is in progress.\nsee \"{}/#WSAEINPROGRESS\" ", ERR_CODES)?,
            TasksLimitReached => write!(f, "A limit on the number of tasks supported by the Windows Sockets implementation has been reached.\nsee \"{}/#WSAEPROCLIM\" ", ERR_CODES)?,
            InvalidData => write!(f, "The lpWSAData parameter is not a valid pointer.\nsee \"{}/#WSAEFAULT\" ", ERR_CODES)?,
        }

        writeln!(f, "for more information")
    }
}

impl From<i32> for WsaError {
    fn from(err_code: i32) -> Self {
        match err_code {
            10091 => SystemNotReady,
            10092 => VersionNotSupported,
            10036 => OperationInProgress,
            10067 => TasksLimitReached,
            10014 => InvalidData,
            _ => UnknownError,
        }
    }
}

/// Initializes `WSA`, calls `WSAStartup` upon initialization, builder for the [`Wsa`] unit struct
pub struct WsaInitializer {
    version: u16,
    data: WSADATA,
}

/// Control flow, makes sure you clean up `WSA` when you finnish using it
#[must_use = "You should clean up after yourself, see `.raii` and `.clean`"]
pub struct Wsa(());

/// Calls `WSACleanup` on drop
pub struct WsaRaii(());

impl Default for WsaInitializer {
    fn default() -> Self {
        Self {
            version: make_version(2, 2),
            data: unsafe { std::mem::zeroed() },
        }
    }
}

impl WsaInitializer {
    /// Sets the version for WSA to be initialized with
    pub fn version(&mut self, new: u16) -> &mut Self {
        self.version = new;
        self
    }

    /// Sets the data to be given when WSA is initialized
    pub fn data(&mut self, new: WSADATA) -> &mut Self {
        self.data = new;
        self
    }

    /// Initializes WSA by calling `WSAStartup`
    /// # Errors
    /// Returns a [`WsaError`] if the the initialization fails
    pub fn init(mut self) -> Result<Wsa> {
        // WSAStartup(u16, *mut WSADATA) -> i32, reminder: UnsafeCell
        let result = unsafe { win::WSAStartup(self.version, &mut self.data as *mut _) };
        if result == 0 {
            Ok(Wsa(()))
        } else {
            Err(result.into())
        }
    }
}

impl Wsa {
    /// Cleans up WSA on drop.  
    /// Takes ownership of self to assert WSA was initialized and to avoid double cleanup.
    #[allow(clippy::must_use_candidate, clippy::unused_self)]
    pub const fn raii(self) -> WsaRaii {
        WsaRaii(())
    }

    /// cleans WSA.  
    /// Takes self to assert WSA was initialized and to avoid double cleanup.
    #[allow(clippy::missing_const_for_fn)]
    pub fn clean(self) {
        self.raii();
    }
}

impl Drop for WsaRaii {
    fn drop(&mut self) {
        // TODO: Find a way to use result
        let _ = unsafe { win::WSACleanup() };
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
