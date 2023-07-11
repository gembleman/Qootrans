use libloading::Library;
use libloading::Symbol;
use std::os::raw::{c_char, c_int, c_ushort};
use std::ffi::CString;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::os::windows::ffi::OsStrExt;
use std::mem;
use std::slice;

//use Path;
fn convert_str_to_char(name: &str) -> *mut c_char {
    CString::new(name).expect("convert_str_to_char faild").into_raw()
}

unsafe fn free_memory(pointer: *const c_ushort, length: usize) {
    let _boxed = Box::from_raw(slice::from_raw_parts_mut(pointer as *mut c_ushort, length) as *mut [c_ushort]);
}

fn convert_str_to_c_ushort(string: &str) -> *const c_ushort {
    // String을 OsString으로 변환하고 와이드 문자로 전환합니다.
    let os_string = OsString::from(string);
    let wide_chars: Vec<u16> = os_string.encode_wide().collect();

    // Vec를 Box<[c_ushort]>로 변환합니다.
    let boxed_slice: Box<[c_ushort]> = wide_chars.into_boxed_slice();
    let pointer: *const c_ushort = boxed_slice.as_ptr();
    
    // 메모리가 해제되지 않도록 멤버를 기억합니다.
    mem::forget(boxed_slice);
    
    pointer
}

unsafe fn convert_c_ushort_to_str(wchar_ptr: *const c_ushort) -> String {
    let mut len = 0;
    
    while *wchar_ptr.add(len) != 0 {
        len += 1;
    }
    let wide_slice = std::slice::from_raw_parts(wchar_ptr, len);
    let os_string = OsString::from_wide(wide_slice);
    os_string.into_string().expect("Failed to convert to string")
    
}

unsafe fn j2k_initialize_ex_init(lib: Library) -> Library{
    
    let j2k_initialize_ex: Symbol<extern "C" fn(*mut c_char, *mut c_char) -> bool> = lib.get(b"J2K_InitializeEx").expect("Failed to load function J2K_InitializeEx");
    
    let raw1: *mut c_char = convert_str_to_char("CSUSER123455");
    let raw2: *mut c_char = convert_str_to_char("C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\Dat");
    let plag = j2k_initialize_ex(raw1, raw2);
    println!("Function loaded {}", plag);
    lib
    
}
unsafe fn j2k_translate_mmntw_funtion(lib: Library, string: &str) -> String {
    let raw3: c_int = 0;
    
    let raw4 = convert_str_to_c_ushort(string);
    println!("j2k_translate_mmntw sucessfully loaded1");
    let j2k_translate_mmntw: Symbol<extern "C" fn(c_int, *const c_ushort) -> *const c_ushort> = lib.get(b"J2K_TranslateMMNTW").expect("Failed to load function");
    println!("j2k_translate_mmntw sucessfully loaded2");
    let plag2_ptr = j2k_translate_mmntw(raw3, raw4);

    let translated_text = convert_c_ushort_to_str(plag2_ptr);
    println!("Function loaded {}", translated_text);
    let length = string.chars().count();
    free_memory(raw4, length);
    translated_text
    
}

fn main() {
    unsafe{
    let lib = Library::new("C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\J2KEngine.dll").unwrap();
    
    println!("Library loaded");
    let wanted_text = "雨が降る 雨が降るようだ";
    let lib = j2k_initialize_ex_init(lib);

    let plag2 = j2k_translate_mmntw_funtion(lib, wanted_text);
    
    println!("translated_text {}", plag2);
    
    }
}
