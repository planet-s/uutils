extern crate wild;

pub fn args() -> impl Iterator<Item=String> {
    wild::args()
}

#[cfg(feature = "libc")]
pub extern crate libc;
#[cfg(feature = "winapi")]
pub extern crate winapi;
#[cfg(feature = "failure")]
extern crate failure;
#[cfg(feature = "failure_derive")]
#[macro_use]
extern crate failure_derive;

#[macro_use]
mod macros;

#[macro_use]
pub mod coreopts;

pub mod panic;

#[cfg(feature = "fs")]
pub mod fs;
#[cfg(feature = "encoding")]
pub mod encoding;
#[cfg(feature = "parse_time")]
pub mod parse_time;

#[cfg(all(not(windows), feature = "mode"))]
pub mod mode;
#[cfg(all(unix, not(target_os = "fuchsia"), feature = "utmpx"))]
pub mod utmpx;
#[cfg(all(unix, not(target_os = "redox"), feature = "entries"))]
pub mod entries;
#[cfg(all(unix, feature = "process"))]
pub mod process;
#[cfg(all(unix, not(target_os = "fuchsia"), feature = "signals"))]
pub mod signals;

#[cfg(all(windows, feature = "wide"))]
pub mod wide;
