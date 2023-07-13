#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use dlopen::symbor::Container;
use dlopen::symbor::{SymBorApi, Symbol};
use dlopen_derive::SymBorApi;
use std::ffi::{CStr, OsStr, OsString};
use std::os::raw::{c_char, c_int};
use std::os::windows::ffi::{OsStrExt, OsStringExt};
use std::slice;

#[derive(SymBorApi)]
pub struct EzTransLib<'a> {
    J2K_InitializeEx: Symbol<'a, J2K_InitializeEx>,
    J2K_TranslateMMNTW: Symbol<'a, J2K_TranslateMMNTW>,
    J2K_Terminate: Symbol<'a, J2K_Terminate>,
}
//c_ushort

type J2K_InitializeEx = unsafe extern "stdcall" fn(*const c_char, *const c_char) -> bool;
type J2K_TranslateMMNTW = unsafe extern "stdcall" fn(c_int, *const u16) -> *const u16;
type J2K_Terminate = unsafe extern "stdcall" fn() -> c_int;

pub enum jp_str_enum<'a> {
    str(&'a str),
    string(&'a String),
}

impl<'a> EzTransLib<'a> {
    /// return false when failed
    pub unsafe fn initialize(&self, init_str: &CStr, home_dir: &CStr) -> bool {
        let ret = (self.J2K_InitializeEx)(init_str.as_ptr(), home_dir.as_ptr());
        println!("엔진 초기화 확인: {}", ret);
        ret
    }

    #[inline]
    pub unsafe fn translate(&self, jp_str: jp_str_enum) -> String {
        let os_str = match jp_str {
            jp_str_enum::str(str) => OsStr::new(str),
            jp_str_enum::string(string) => OsStr::new(string),
        };

        let input_str: Vec<u16> = os_str.encode_wide().chain(Some(0)).collect();
        let ret = (self.J2K_TranslateMMNTW)(0, input_str.as_ptr());

        let mut current_ptr = ret;
        let mut len = 0;
        //문자열 길이 계산 - 포인터를 뒤로 이동시키며 0을 만날 때까지
        while *current_ptr != 0 {
            len += 1;
            current_ptr = current_ptr.add(1);
        }

        let u16_slice = slice::from_raw_parts(ret, len);
        let os_string = OsString::from_wide(u16_slice);
        let strrr = os_string
            .into_string()
            .expect("Failed to convert to string");
        //rintln!("funtion String: {}", strrr);

        strrr
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
