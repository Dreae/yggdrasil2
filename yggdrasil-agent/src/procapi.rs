#[cfg(windows)]
use kernel32;
#[cfg(windows)]
use winapi::winnt::{HANDLE, PROCESS_ALL_ACCESS};

#[cfg(unix)]
use libc::waitpid;

#[cfg(windows)]
pub fn process_running(pid: u32) -> bool {
    unsafe {
        let handle = kernel32::OpenProcess(PROCESS_ALL_ACCESS, 0, pid);
        let mut code: u32 = 0;
        kernel32::GetExitCodeProcess(handle, &mut code);
        return code == 259;
    }
}

#[cfg(unix)]
pub fn process_running(pid: u32) -> bool {
    unsafe {
        let mut code: i32 = 0;
        let return_value = waitpid(pid as i32, &mut code, 0);
        return return_value == 0;
    }
}
