mod logging;

use std::{error::Error, ffi::{c_void, CString, OsString}, os::windows::ffi::OsStringExt, path::{Path, PathBuf}, sync::OnceLock};

use winapi::{shared::minwindef::{DWORD, HINSTANCE, LPVOID}, um::{libloaderapi::GetModuleFileNameW, winnt::DLL_PROCESS_ATTACH, winuser::MessageBoxA}};

static DLL_PATH: OnceLock<PathBuf> = OnceLock::new();
static APP_ID: OnceLock<u32> = OnceLock::new();

#[unsafe(no_mangle)]
extern "system" fn DllMain(module: HINSTANCE, reason: DWORD, _reserved: LPVOID) -> bool {    
    match reason {
        DLL_PROCESS_ATTACH => {
            let dll_path = {
                let mut buffer = [0u16; 1024];
                let len = unsafe { GetModuleFileNameW(module, buffer.as_mut_ptr(), buffer.len() as u32) };
                let path = OsString::from_wide(&buffer[..len as usize]).into_string().unwrap();
                let path = Path::new(&path);
                path.parent().unwrap().to_path_buf()
            };
            DLL_PATH.set(dll_path).ok();

            logging::init_logger();
            logging::setup_panic_handler();
        }
        _ => {}
    }
    
    true
}

#[repr(C)]
pub struct IGameTokenInterface {
    vtable: *const IGameTokenInterfaceVtable,
}

#[repr(C, align(32))]
pub struct IGameTokenInterfaceVtable {
    is_token_loaded: *const c_void,
    return_0: *const c_void,
    get_cached_or_fresh_token: *const c_void,
    invalidate_cached_token: *const c_void,
    get_buffer: *const c_void,
    new_thread_get_burn_ticket_res: *const c_void,
    get_thread: *const c_void,
}

#[unsafe(export_name = "?getGameTokenInterface@@YAPEAVIGameTokenInterface@@PEAX_K@Z")]
pub extern "C" fn get_game_token_interface(
    app_id: *const i64,
    always_8: i64,
) -> *const IGameTokenInterface {
    let app_id = unsafe { *app_id };

    APP_ID.set(app_id as u32).ok();

    log::info!("getGameTokenInterface called {:?} {:?}", app_id, always_8);

    let vtable = Box::new(IGameTokenInterfaceVtable {
        is_token_loaded: is_token_loaded as *const c_void,
        return_0: return_0 as *const c_void,
        get_cached_or_fresh_token: get_cached_or_fresh_token as *const c_void,
        invalidate_cached_token: invalidate_cached_token as *const c_void,
        get_buffer: get_buffer as *const c_void,
        new_thread_get_burn_ticket_res: new_thread_get_burn_ticket_res as *const c_void,
        get_thread: get_thread as *const c_void,
    });

    let example = Box::new(IGameTokenInterface {
        vtable: Box::into_raw(vtable),
    });

    Box::into_raw(example)
}

fn is_token_loaded(
    this: *const IGameTokenInterface
) -> u32 {
    log::info!("get_internal_8 called {:?}", this);

    1
}

fn return_0() -> i64 {
    log::info!("return_0 called");
    
    0
}

fn get_cached_or_fresh_token(
    this: *mut IGameTokenInterface,
    token_buffer_ptr: *const c_void,
    not_used: i32
) -> bool {
    log::info!("get_cached_or_fresh_token called {:?} {:?} {:?}", this, token_buffer_ptr, not_used);

    if !token_exists() {
        unsafe {
            let request_token = std::ffi::CStr::from_ptr(token_buffer_ptr as *const i8).to_str().unwrap();
            let request_token = format!("{}|{}", request_token, APP_ID.get().unwrap());
            let path = DLL_PATH.get().unwrap()
                .join("token_req.txt");
            std::fs::write(path, request_token).unwrap();
    
            message_box("Info", "Token request generated, please check the 'token_req.txt' file in the current directory. If you have a token, place it in a file named 'token.txt' in the same directory.");
            std::process::exit(0);
        }
    }

    true
}

fn invalidate_cached_token(
    this: *const IGameTokenInterface
) {
    log::info!("invalidate_cached_token called {:?}", this);

    message_box("Info", "Denuvo tried to delete the token, if this error persists your token might have become invalid");
}

fn get_buffer(
    this: *const IGameTokenInterface,
    lenght: *mut u64
) -> *const c_void {
    log::info!("get_buffer called {:?} {:?}", this, lenght);

    let token = get_token()
        .map_err(|e| {
            message_box("Error", &format!("Error getting token: {}", e));
        })
        .unwrap();

    if !lenght.is_null() {
        unsafe {
            *lenght = token.len() as u64;
        }
    }

    Box::into_raw(token.into_boxed_str()) as *const c_void
}

fn new_thread_get_burn_ticket_res(
    this: *const IGameTokenInterface,
    param: i64
) -> *const c_void {
    log::info!("new_thread_get_burn_ticket_res called {:?} {:?}", this, param);

    std::ptr::null()
}

fn get_thread(
    this: *const IGameTokenInterface,
    param: *mut u64
) -> *const c_void {
    log::info!("get_thread called {:?} {:?}", this, param);

    std::ptr::null()
}

pub fn message_box(
    title: &str,
    message: &str
) {
    let title = CString::new(title).unwrap();
    let message = CString::new(message).unwrap();

    unsafe {
        MessageBoxA(
            std::ptr::null_mut(),
            message.as_ptr() as *const i8,
            title.as_ptr() as *const i8,
            0,
        );
    }
}

fn get_token() -> Result<String, Box<dyn Error>> {
    let path = DLL_PATH.get().ok_or("DLL_PATH not set")?
        .join("token.txt");
    let token = std::fs::read_to_string(path)?
        .trim()
        .to_string();
    
    Ok(token)
}

fn token_exists() -> bool {
    DLL_PATH.get().unwrap_or(&PathBuf::from(""))
        .join("token.txt")
        .exists()
}
