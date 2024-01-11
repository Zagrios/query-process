use napi_derive::napi;
use napi::Error;
use Result::Err;

#[cfg(target_os = "windows")]
use windows::Win32::{
    Foundation::{HANDLE, GetLastError, CloseHandle},
    System::Threading::{OpenProcess, OpenProcessToken, PROCESS_QUERY_LIMITED_INFORMATION},
    Security::{GetTokenInformation, TokenElevation, TOKEN_ELEVATION, TOKEN_QUERY}
};

#[cfg(target_os = "windows")]
use std::mem::size_of;
#[cfg(target_os = "windows")]
use std::ffi::c_void;

#[cfg(target_os = "windows")]
struct SafeHandle(HANDLE);

#[cfg(target_os = "windows")]
impl Drop for SafeHandle {
    fn drop(&mut self) {
        unsafe { let _ = CloseHandle(self.0); };
    }
}

#[cfg(target_os = "linux")]
use nix::unistd::{Pid, Uid};
#[cfg(target_os = "linux")]
use nix::libc::getuid;
#[cfg(target_os = "linux")]
use procfs::process::Process;

#[napi]
pub fn is_elevated(pid: u32) -> Result<bool, Error> {

    #[cfg(target_os = "windows")]
    unsafe {

        let process_handle = match OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) {
            Ok(handle) => SafeHandle(handle),
            Err(_) => return Err(Error::from_reason(format!("OpenProcess failed with error code: {:?}", GetLastError())))
        };

        let mut token_handle: SafeHandle = SafeHandle(HANDLE::default());

        if OpenProcessToken(process_handle.0, TOKEN_QUERY, &mut token_handle.0).is_err() {
            return Err(Error::from_reason(format!("OpenProcessToken failed with error code: {:?}", GetLastError())));
        }

        let mut elevation = TOKEN_ELEVATION::default();
        let mut size = size_of::<TOKEN_ELEVATION>() as u32;

        if GetTokenInformation(token_handle.0, TokenElevation, Some(&mut elevation as *mut _ as *mut c_void), size, &mut size).is_err() {
            return Err(Error::from_reason(format!("GetProcessInformation failed with error code: {:?}", GetLastError())));
        }

        return Ok(elevation.TokenIsElevated != 0);

    }

    #[cfg(target_os = "linux")]
    {
        let process = match Process::new(pid as i32) {
            Ok(proc) => proc,
            Err(e) => return Err(Error::from_reason(format!("Unable to build process from pid: {}", e)))
        };

        return match process.uid() {
            Ok(uid) => Ok(Uid::is_root(Uid::from_raw(uid))),
            Err(e) => Err(Error::from_reason(format!("Unable to get process uid: {}", e)))
        };
    }

    #[cfg(not(any(target_os = "windows", target_os = "linux")))]
    {
        return Err(Error::from_reason("Unsupported OS"));
    }

}
