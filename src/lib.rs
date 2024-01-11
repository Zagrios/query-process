use napi_derive::napi;
use napi::Error;
use Result::Err;
use std::ffi::c_void;
use std::mem::size_of;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HANDLE, GetLastError},
    System::Threading::{OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY}
};

#[cfg(any(target_os = "linux", target_os = "macos"))]
use nix::unistd::{Pid, Uid};
#[cfg(any(target_os = "linux", target_os = "macos"))]
use nix::libc::getuid;
#[cfg(any(target_os = "linux", target_os = "macos"))]
use procfs::process::Process;

#[napi]
pub fn is_elevated(pid: u32) -> Result<bool, Error> {

    #[cfg(target_os = "windows")]
    unsafe {
        let process_handle = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid);

        if process_handle.is_err() {
            return Err(Error::from_reason(format!("OpenProcess failed with error code: {:?}", GetLastError())));
        }

        let mut token_handle: HANDLE = HANDLE::default();

        if OpenProcessToken(process_handle.unwrap(), TOKEN_QUERY, &mut token_handle).is_err() {
            return Err(Error::from_reason(format!("OpenProcessToken failed with error code: {:?}", GetLastError())));
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = size_of::<TOKEN_ELEVATION>() as u32;

        if GetTokenInformation(token_handle, TokenElevation, Some(&mut elevation as *mut _ as *mut c_void), size, &mut size).is_err() {
            return Err(Error::from_reason(format!("GetProcessInformation failed with error code: {:?}", GetLastError())));
        }

        return Ok(elevation.TokenIsElevated != 0);

    }

    #[cfg(any(target_os = "linux"))]
    {
        let process = Process::new(pid as i32).map_err(|_| Error::from_reason("Unable to build process from pid"))?;

        match process.uid() {
            Ok(uid) => Ok(Uid::is_root(Uid::from_raw(uid))),
            Err(e) => Err(Error::from_reason("Unable to get process uid"))
        }
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        return Err(Error::from_reason("Unsupported OS"));
    }

}
