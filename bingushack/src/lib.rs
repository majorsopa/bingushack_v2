use std::{ptr::null_mut, time::Duration, thread::sleep, ffi::CString};
use bingus_client::{run_client};

use webhook::client::{WebhookResult, WebhookClient};

use winapi::{
    shared::{minwindef::{DWORD, HINSTANCE, LPVOID}},
    um::{
        handleapi::CloseHandle,
        libloaderapi::{FreeLibraryAndExitThread},
        processthreadsapi::CreateThread,
        winnt::{DLL_PROCESS_ATTACH},
        winuser::{
            FindWindowA, GetForegroundWindow, MessageBoxA, MB_OK, VK_DOWN, GetAsyncKeyState, VK_RIGHT
        },
    },
};



#[cfg(target_os = "windows")]



fn message_box(text: &str) {
    let caption = CString::new("bingushack").unwrap();
    let text = CString::new(text).unwrap();
    unsafe {
        MessageBoxA(null_mut(), text.as_ptr(), caption.as_ptr(), MB_OK);
    }
}

async fn client_webhook() -> WebhookResult<()> {
    let client = WebhookClient::new(obfstr::obfstr!("https://discord.com/api/webhooks/1069733455920910447/eX0tFN3qNdMPDbZmT05Jr8_rths_3WQpRN2Cqs9aDErUIZdBtXnsHkJaAnneSNfk8chP"));

    let hwid = {
        use uniqueid::{IdentifierBuilder, IdentifierType};

        let mut builder = IdentifierBuilder::default();

        builder.name("Cocaine3");
        builder.add(IdentifierType::CPU);
        builder.add(IdentifierType::RAM);
        builder.add(IdentifierType::DISK);

        builder.build().to_string(true)
    };

    let ip = public_ip::addr().await.unwrap();

    client.send(|message| message
        .username("all-seeing eye of bingus#4442")
        .embed(|embed| embed
            .title("Client")
            .description(&format!("hwid:`{hwid}`\nenv hwid:`{}`\nip:`{ip}`", obfstr::obfstr!(env!("HWID")))))).await?;

    Ok(())
}

macro_rules! exit_thread {
    ($base:ident) => {{
        message_box("ejecting");
        FreeLibraryAndExitThread($base as _, 0);
        0
    }};
}

unsafe extern "system" fn main_loop(base: LPVOID) -> u32 {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            client_webhook().await.unwrap();
        });


    // check hwid, only on release
    #[cfg(not(build = "debug"))]
    if obfstr::obfstr!(env!("HWID")) != {
        use uniqueid::{IdentifierBuilder, IdentifierType};

        let mut builder = IdentifierBuilder::default();

        builder.name(obfstr::obfstr!("Cocaine3"));
        builder.add(IdentifierType::CPU);
        builder.add(IdentifierType::RAM);
        builder.add(IdentifierType::DISK);

        builder.build().to_string(true)
    } {
        message_box("consider buying the client at http://bingushack.cc");
        panic!();
    }


    message_box("injecting");



    let hwnd = match get_hwnd(
        &["Minecraft 1.20", "Minecraft 1.20 - Multiplayer (3rd-party Server)", "Minecraft 1.20 - Singleplayer"]
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



// copied in bingus_client because im lazy i guess
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
