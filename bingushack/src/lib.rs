use std::{ptr::null_mut, time::Duration, thread::sleep, ffi::CString};
use bingus_client::run_client;
use winapi::{
    shared::minwindef::{DWORD, HINSTANCE, LPVOID},
    um::{
        handleapi::CloseHandle,
        libloaderapi::FreeLibraryAndExitThread,
        processthreadsapi::CreateThread,
        winnt::DLL_PROCESS_ATTACH,
        winuser::{
            FindWindowA, GetForegroundWindow, MessageBoxA, MB_OK, VK_DOWN, GetAsyncKeyState, VK_RIGHT
        },
    },
};

#[cfg(target_os = "windows")]


pub fn message_box(text: &str) {
    let caption = CString::new("bingushack").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}

macro_rules! exit_thread {
    ($base:ident) => {{
        message_box("ejecting");
        FreeLibraryAndExitThread($base as _, 0);
        0
    }};
}

unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {
    message_box("injected");



    let hwnd = match get_hwnd(
        &["Minecraft 1.19.3", "Minecraft 1.19.3 - Multiplayer (3rd-party Server)", "Minecraft 1.19.3 - Singleplayer"]
    ) {
        Some(hwnd) => hwnd,
        None => return exit_thread!(base),
    };

    loop {
        if hwnd != GetForegroundWindow() {
            sleep(Duration::from_millis(50));
            continue;
        }

        if GetAsyncKeyState(VK_DOWN) & 0x01 == 1 {
            break;
        }


        if GetAsyncKeyState(VK_RIGHT) & 0x01 == 1 {
            run_client();
        }
    }

    exit_thread!(base)
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
        let window_name = CString::new(*window_name).unwrap();
        let hwnd = FindWindowA(null_mut(), window_name.as_ptr());
        if !hwnd.is_null() {
            return Some(hwnd);
        }
    }
    None
}

