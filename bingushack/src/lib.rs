use std::ptr::null_mut;
use winapi::{
    shared::minwindef::{DWORD, HINSTANCE, LPVOID},
    um::{
        handleapi::CloseHandle,
        libloaderapi::FreeLibraryAndExitThread,
        processthreadsapi::CreateThread,
        winnt::DLL_PROCESS_ATTACH,
        winuser::{
            FindWindowA
        },
    },
};

#[cfg(target_os = "windows")]


unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {


    FreeLibraryAndExitThread(base as _, 0);
    0
}



#[no_mangle]
pub extern "stdcall" fn DllMain(
    hinst_dll: HINSTANCE,
    fdw_reason: DWORD,
    _lpv_reserved: LPVOID,
) -> i32 {
    match fdw_reason {
        DLL_PROCESS_ATTACH => {
            unsafe {
                let bingus_thread = CreateThread(
                    null_mut(),
                    0,
                    Some(main_loop),
                    hinst_dll as _,
                    0,
                    null_mut(),
                );
                CloseHandle(bingus_thread);
            }
            true as i32
        }
        _ => true as i32, // it went a-ok because we dont know what happened so lol fuck off
    }
}

unsafe fn get_hwnd(window_names: &[&str]) -> Option<winapi::shared::windef::HWND> {
    for window_name in window_names {
        let hwnd = FindWindowA(null_mut(), window_name.as_ptr() as _);
        if !hwnd.is_null() {
            return Some(hwnd);
        }
    }
    None
}

