//#![allow(non_camel_case_types)]
//#![allow(non_snake_case)]

//use libc::{c_char, c_int, c_void, c_ushort};
pub use dlopen::symbor::Container;
use dlopen::symbor::{SymBorApi, Symbol};
use dlopen_derive::SymBorApi;
use std::os::raw::{c_char, c_int, c_ushort};

use std::ffi::OsString;
use std::ffi::{CStr, OsStr};
use std::os::windows::ffi::OsStrExt;

#[derive(SymBorApi)]
pub struct EzTransLib<'a> {
    J2K_InitializeEx: Symbol<'a, J2K_InitializeEx>,
    J2K_TranslateMMNTW: Symbol<'a, J2K_TranslateMMNTW>,
    J2K_Terminate: Symbol<'a, J2K_Terminate>,
}
//c_ushort

type J2K_InitializeEx = unsafe extern "stdcall" fn(*const c_char, *const c_char) -> bool;
type J2K_TranslateMMNTW = unsafe extern "stdcall" fn(c_int, *const c_ushort) -> *const c_ushort;
type J2K_Terminate = unsafe extern "stdcall" fn() -> c_int;

impl<'a> EzTransLib<'a> {
    /// return false when failed
    pub unsafe fn initialize(&self, init_str: &CStr, home_dir: &CStr) -> bool {
        let ret = (self.J2K_InitializeEx)(init_str.as_ptr(), home_dir.as_ptr());
        println!("엔진 초기화 확인: {}", ret);
        ret == true
    }

    #[inline]
    pub unsafe fn translate(&self, jp_str: String) -> *const u16 {
        let input_str: Vec<u16> = OsString::from(jp_str).encode_wide().collect();
        let ret = (self.J2K_TranslateMMNTW)(0, input_str.as_ptr());
        ret
        //EzString(CStr::from_ptr(ret))
    }

    #[inline]
    pub unsafe fn terminate(&self) {
        (self.J2K_Terminate)();
    }
}

pub unsafe fn load_library(
    dll_path: impl AsRef<OsStr>,
) -> Result<Container<EzTransLib<'static>>, dlopen::Error> {
    Container::load(dll_path)
}

/*
pub struct EzString(&'static CStr);

impl Drop for EzString {
    fn drop(&mut self) {
        unsafe {
            libc::free(self.0.as_ptr() as *mut c_char as *mut c_void);
        }
    }
}

impl EzString {
    pub fn as_bytes(&self) -> &[u8] {
        self.0.to_bytes()
    }
}
*/
