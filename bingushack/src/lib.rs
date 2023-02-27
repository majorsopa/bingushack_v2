mod agent;

use std::{ptr::null_mut, time::Duration, thread::sleep, ffi::CString, sync::Once};
use bingus_client::{run_client, MODULES};
use bingus_module::prelude::*;
use webhook::client::{WebhookResult, WebhookClient};
use widestring::WideCString;
use winapi::{
    shared::{minwindef::{DWORD, HINSTANCE, LPVOID, HMODULE}, windef::{HDC, HGLRC__}},
    um::{
        handleapi::CloseHandle,
        libloaderapi::{FreeLibraryAndExitThread, GetModuleHandleW, GetProcAddress},
        processthreadsapi::CreateThread,
        winnt::{DLL_PROCESS_ATTACH, DLL_PROCESS_DETACH},
        winuser::{
            FindWindowA, GetForegroundWindow, MessageBoxA, MB_OK, VK_DOWN, GetAsyncKeyState, VK_RIGHT
        }, wingdi::{wglGetCurrentContext, wglCreateContext, wglMakeCurrent, wglGetProcAddress},
    },
};
use jvmti::{
    agent::Agent,
    config::Config,
    context::static_context,
    native::{JavaVMPtr, MutString, ReturnValue, VoidPtr},
    options::Options,
};
use once_cell::sync::OnceCell;
use std::sync::atomic::AtomicPtr;

#[cfg(target_os = "windows")]



static FIRST_RENDER: Once = Once::new();
static mut NEW_CONTEXT: OnceCell<AtomicPtr<HGLRC__>> = OnceCell::new();
static mut OLD_CONTEXT: OnceCell<AtomicPtr<HGLRC__>> = OnceCell::new();


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

        builder.name("Cocaine3");
        builder.add(IdentifierType::CPU);
        builder.add(IdentifierType::RAM);
        builder.add(IdentifierType::DISK);

        builder.build().to_string(true)
    } {
        message_box("consider buying the client at http://bingushack.cc");
        panic!();
    }


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
            //crochet::enable!(swapbuffers_hook).expect("could not enable swapbuffers hook");
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
        DLL_PROCESS_DETACH => {
            //crochet::disable!(swapbuffers_hook).expect("could not disable swapbuffers hook");

            true as i32
        }
        _ => true as i32, // it went a-ok because we dont know what happened so lol fuck off
    }
}


/*
// causes some issues with ejection
#[crochet::hook("opengl32.dll", "wglSwapBuffers")]
fn swapbuffers_hook(hdc: HDC) -> winapi::ctypes::c_int {
    FIRST_RENDER.call_once(|| {
        // initialize opengl shit
        unsafe {
            let _ = OLD_CONTEXT.get_or_init(|| AtomicPtr::new(wglGetCurrentContext()));
            let _ = NEW_CONTEXT.get_or_init(|| AtomicPtr::new(wglCreateContext(hdc)));

            let local_new_context = NEW_CONTEXT.get_mut().unwrap();
            wglMakeCurrent(hdc, *local_new_context.get_mut());
        }

        let opengl32_module: HMODULE;
        let opengl32_str = WideCString::from_str("opengl32.dll").unwrap();

        unsafe {
            opengl32_module = GetModuleHandleW(opengl32_str.as_ptr());
        }
        if opengl32_module == null_mut() {
            message_box("opengl32.dll not found. what the fuck did you do??");
        }

        gl::load_with(|s| unsafe {
            let gl_fn_cstr = CString::new(s).unwrap();
            let gl_fn_cstr_ptr = gl_fn_cstr.as_ptr();  // this is unneeded
            let check = wglGetProcAddress(gl_fn_cstr_ptr);
            if check == null_mut() {
                GetProcAddress(opengl32_module, gl_fn_cstr_ptr)
            } else {
                check
            }
        } as *const _);
    });

    if let Some(modules) = MODULES.get() {
        unsafe {
            let local_new_context = NEW_CONTEXT.get_mut().unwrap();
            wglMakeCurrent(hdc, *local_new_context.get_mut());
        }
        for module in modules.lock().unwrap().iter_mut() {
            if *module.get_enabled().0.get_bool() {
                module.render();
            }
        }
    }

    unsafe {
        let local_old_context = OLD_CONTEXT.get_mut().unwrap();
        wglMakeCurrent(hdc, *local_old_context.get_mut());  // might be bad?
    }

    call_original!(hdc)
}
*/



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


// where the agent magic happens
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn Agent_OnLoad(
    vm: JavaVMPtr,
    options: MutString,
    reserved: VoidPtr,
) -> ReturnValue {
    let options = Options::parse(stringify(options));

    if let Some(config) = Config::read_config() {
        static_context().set_config(config);
    }

    let mut agent = Agent::new(vm);

    agent.on_garbage_collection_start(Some(on_garbage_collection_start));
    agent.on_garbage_collection_finish(Some(on_garbage_collection_finish));
    agent.on_vm_object_alloc(Some(on_object_alloc));
    agent.on_vm_object_free(Some(on_object_free));
    agent.on_class_file_load(Some(on_class_file_load));
    agent.on_method_entry(Some(on_method_entry));
    agent.on_method_exit(Some(on_method_exit));
    agent.on_thread_start(Some(on_thread_start));
    agent.on_thread_end(Some(on_thread_end));
    agent.on_monitor_wait(Some(on_monitor_wait));
    agent.on_monitor_waited(Some(on_monitor_waited));
    agent.on_monitor_contended_enter(Some(on_monitor_contended_enter));
    agent.on_monitor_contended_entered(Some(on_monitor_contended_entered));
    agent.on_class_file_load(Some(on_class_file_load));

    agent.update();

    return 0;
}

#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "C" fn Agent_OnUnload(vm: JavaVMPtr) {}
