use dbus;
use std::io;

#[derive(Debug, Error)]
pub enum DaemonError {
    #[error(display = "failed to make a private dbus connection to the system bus: {}", _0)]
    PrivateConnection(dbus::Error),
    #[error(display = "failed to process dbus events: {}", _0)]
    Process(dbus::Error),
    #[error(display = "failed to register dbus name: {}", _0)]
    RegisterName(dbus::Error),
    #[error(display = "failed to register object paths in the dbus tree: {}", _0)]
    TreeRegister(dbus::Error),
    #[error(display = "failure to create {}: {}", _0, _1)]
    VarLibDirectory(&'static str, io::Error),
}
