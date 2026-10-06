#[allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code,
    clippy::all
)]
pub mod Microsoft {
    pub mod Web {
        pub mod WebView2 {
            pub mod Win32 {
                pub(crate) mod windows_link {
                    use core::ffi::c_void;
                    use std::sync::atomic::{AtomicPtr, Ordering};

                    extern "system" {
                        fn LoadLibraryA(lpLibFileName: *const u8) -> *mut c_void;
                        fn LoadLibraryW(lpLibFileName: *const u16) -> *mut c_void;
                        fn GetProcAddress(hModule: *mut c_void, lpProcName: *const u8) -> *mut c_void;
                    }

                    static LOADED_MODULE: AtomicPtr<c_void> = AtomicPtr::new(core::ptr::null_mut());

                    pub fn get_module() -> *mut c_void {
                        let handle = LOADED_MODULE.load(Ordering::Acquire);
                        if !handle.is_null() {
                            return handle;
                        }

                        let mut module = unsafe { LoadLibraryA(b"WebView2Loader.dll\0".as_ptr()) };

                        if module.is_null() {
                            if let Some(path) = unpack_embedded_loader() {
                                use std::os::windows::ffi::OsStrExt;
                                let wide: Vec<u16> = path.as_os_str().encode_wide().chain(Some(0)).collect();
                                module = unsafe { LoadLibraryW(wide.as_ptr()) };
                            }
                        }

                        if !module.is_null() {
                            LOADED_MODULE.store(module, Ordering::Release);
                        }
                        module
                    }

                    fn unpack_embedded_loader() -> Option<std::path::PathBuf> {
                        let temp = std::env::temp_dir().join("mercy_runtime");
                        let _ = std::fs::create_dir_all(&temp);
                        let dll_path = temp.join("WebView2Loader.dll");
                        const BYTES: &[u8] = include_bytes!("../../../assets/windows/WebView2Loader.dll");
                        let write_needed = match std::fs::metadata(&dll_path) {
                            Ok(m) => m.len() != BYTES.len() as u64,
                            Err(_) => true,
                        };
                        if write_needed {
                            let _ = std::fs::write(&dll_path, BYTES);
                        }
                        Some(dll_path)
                    }

                    pub unsafe fn resolve_proc(name: *const u8) -> *mut c_void {
                        let module = get_module();
                        if module.is_null() {
                            return core::ptr::null_mut();
                        }
                        unsafe { GetProcAddress(module, name) }
                    }

                    macro_rules! link_webview2 {
                        ($library:literal $abi:literal fn $function:ident ($($arg:ident : $arg_ty:ty),* $(,)?) -> $ret:ty) => (
                            unsafe fn $function($($arg : $arg_ty),*) -> $ret {
                                let proc_name = concat!(stringify!($function), "\0");
                                let proc = unsafe {
                                    $crate::Microsoft::Web::WebView2::Win32::windows_link::resolve_proc(
                                        proc_name.as_ptr(),
                                    )
                                };
                                if proc.is_null() {
                                    return windows_core::HRESULT(0x80004005u32 as i32);
                                }
                                let func: unsafe extern $abi fn($($arg_ty),*) -> $ret = unsafe {
                                    core::mem::transmute(proc)
                                };
                                unsafe { func($($arg),*) }
                            }
                        )
                    }

                    pub(crate) use link_webview2 as link;
                }

                include!("bindings.rs");
            }
        }
    }
}

pub mod declared_interfaces;
