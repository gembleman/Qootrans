mod eztrans_lib;
// `eztrans_lib`는 실제 라이브러리 크레이트 이름입니다.
use eztrans_lib::load_library;
use std::ffi::CString;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::slice;

fn main() {
    // 동적 라이브러리의 경로를 지정합니다.
    let library_path = "C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\J2KEngine.dll";

    // 동적 라이브러리를 로드하고 EzTransLib 인스턴스를 가져옵니다.
    let eztrans_lib = unsafe {
        load_library(library_path)
            .unwrap_or_else(|_| panic!("Error loading library at: {}", library_path))
    };

    // 라이브러리를 초기화합니다.
    let init_str = CString::new("CSUSER123455").unwrap();
    let home_dir = CString::new("C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\Dat").unwrap();
    let initialized = unsafe { eztrans_lib.initialize(&init_str.as_c_str(), &home_dir.as_c_str()) };

    if initialized != true {
        println!("Failed to initialize EzTransLib");
    }
    // 라이브러리 내의 함수를 호출하여 번역합니다.
    let text_big: &str = {
        "プログラマーとデザイナー二人でUnityを使ったゲームを開発中。
    ゲーム開発している経過等をツイートします。
    アップする画像は制作中のものだったりするので色々変わります。"
    };
    //let text_length = text_big.chars().count();
    //enhd 적용됐는지 확인용 - "蜜ドル辞典"
    let text: String = String::from(text_big);

    let translated_string = unsafe { eztrans_lib.translate(text) };

    // 문자열을 &str로 변환합니다..
    println!("Translated string: {:?}", translated_string);
    //let mut len = 0;

    //while unsafe { *translated_string.add(len) } != 0 {
    //    len += 1;
    //}
    /*
    let len = find_len(translated_string);
    // *mut u16 포인터를 &[u16] 슬라이스로 변환
    println!("len: {}", len); //138
    println!("text_length: {}", text_length); //101

    let u16_slice = unsafe { slice::from_raw_parts(translated_string, len) };
    println!("u16_slice: {:?}", u16_slice);
    // UTF-16 슬라이스를 UTF-8으로 디코딩하여 &str을 얻습니다.
    let os_string = OsString::from_wide(u16_slice);
    let strrr = os_string
        .into_string()
        .expect("Failed to convert to string");

    println!("String: {}", strrr);
    */

    // 라이브러리를 종료합니다.
    unsafe { eztrans_lib.terminate() };
}
