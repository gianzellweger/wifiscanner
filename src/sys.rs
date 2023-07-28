#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub(crate) use self::macos::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub(crate) use self::linux::*;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub(crate) use self::windows::*;

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
compile_error!("wifiscan doesn't compile for this platform yet. Use the compile-anyway feature to compile this crate anyway");

#[cfg(all(not(any(target_os = "macos", target_os = "linux", target_os = "windows")), feature = "compile-anyway"))]
mod lib;
#[cfg(all(not(any(target_os = "macos", target_os = "linux", target_os = "windows")), feature = "compile-anyway"))]
pub(crate) fn scan() -> lib::Result<Vec<lib::Wifi>> { Err(UnsupportedPlatform) }
