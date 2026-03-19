#![cfg_attr(windows, feature(abi_vectorcall))]
#[cfg(not(test))]
use ext_php_rs::{prelude::*, wrap_constant};

use ext_php_rs::php_const;

use systemd::daemon;

#[cfg(not(test))]
#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
        .constant(wrap_constant!(SYSTEMD_STATE_BUSERROR))
        .constant(wrap_constant!(SYSTEMD_STATE_ERRNO))
        .constant(wrap_constant!(SYSTEMD_STATE_EXTEND_TIMEOUT_USEC))
        .constant(wrap_constant!(SYSTEMD_STATE_FDNAME))
        .constant(wrap_constant!(SYSTEMD_STATE_FDSTORE))
        .constant(wrap_constant!(SYSTEMD_STATE_FDSTOREREMOVE))
        .constant(wrap_constant!(SYSTEMD_STATE_MAINPID))
        .constant(wrap_constant!(SYSTEMD_STATE_READY))
        .constant(wrap_constant!(SYSTEMD_STATE_RELOADING))
        .constant(wrap_constant!(SYSTEMD_STATE_STATUS))
        .constant(wrap_constant!(SYSTEMD_STATE_STOPPING))
        .constant(wrap_constant!(SYSTEMD_STATE_WATCHDOG))
        .constant(wrap_constant!(SYSTEMD_STATE_WATCHDOG_USEC))
        .function(wrap_function!(notify))
        .function(wrap_function!(watchdog_enabled))
}

#[php_const]
pub const SYSTEMD_STATE_BUSERROR: &str = daemon::STATE_BUSERROR;
#[php_const]
pub const SYSTEMD_STATE_ERRNO: &str = daemon::STATE_ERRNO;
#[php_const]
pub const SYSTEMD_STATE_EXTEND_TIMEOUT_USEC: &str = daemon::STATE_EXTEND_TIMEOUT_USEC;
#[php_const]
pub const SYSTEMD_STATE_FDNAME: &str = daemon::STATE_FDNAME;
#[php_const]
pub const SYSTEMD_STATE_FDSTORE: &str = daemon::STATE_FDSTORE;
#[php_const]
pub const SYSTEMD_STATE_FDSTOREREMOVE: &str = daemon::STATE_FDSTOREREMOVE;
#[php_const]
pub const SYSTEMD_STATE_MAINPID: &str = daemon::STATE_MAINPID;
#[php_const]
pub const SYSTEMD_STATE_READY: &str = daemon::STATE_READY;
#[php_const]
pub const SYSTEMD_STATE_RELOADING: &str = daemon::STATE_RELOADING;
#[php_const]
pub const SYSTEMD_STATE_STATUS: &str = daemon::STATE_STATUS;
#[php_const]
pub const SYSTEMD_STATE_STOPPING: &str = daemon::STATE_STOPPING;
#[php_const]
pub const SYSTEMD_STATE_WATCHDOG: &str = daemon::STATE_WATCHDOG;
#[php_const]
pub const SYSTEMD_STATE_WATCHDOG_USEC: &str = daemon::STATE_WATCHDOG_USEC;

/// Notifies systemd that daemon stat has changed
/// @param bool unset_environment
/// @param string state the state name to change
/// @param string the new value of the state
/// @return bool true if systemd was contacted successfully
#[cfg(not(test))]
#[php_function]
#[php(name = "systemd_notify")]
fn notify(unset_environment: bool, state: &str, value: &str) -> PhpResult<bool> {
    let res = daemon::notify(unset_environment, [&(state, value)].into_iter()).map_err(|_| "failed to notify systemd")?;
    Ok(res)
}

/// @param bool unset_environment
/// @return u64 watchdog timeout in microseconds or 0 if the watchdog is disabled
#[cfg(not(test))]
#[php_function]
#[php(name = "systemd_watchdog_enabled")]
fn watchdog_enabled(unset_environment: bool) -> PhpResult<u64> {
    let res = daemon::watchdog_enabled(unset_environment).map_err(|_| "failed to retrieve watchdog timeout")?;
    Ok(res)
}
