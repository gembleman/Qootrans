mod eztrans_lib;
// `eztrans_lib`는 실제 라이브러리 크레이트 이름입니다.
use dlopen::symbor::Container;
use eztrans_lib::{load_library, EzString, EzTransLib};
use std::ffi::CString;
use std::ffi::OsString;
use std::os::windows::ffi::OsStringExt;
use std::path::Path;
use std::slice;
extern crate encoding;
use encoding::all::WINDOWS_949;
use encoding::{DecoderTrap, Encoding};

fn main() {
    // 동적 라이브러리의 경로를 지정합니다.
    let library_path = "C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\J2KEngine.dll";

    // 동적 라이브러리를 로드하고 EzTransLib 인스턴스를 가져옵니다.
    let eztrans_lib: Container<EzTransLib<'_>> =
        unsafe { load_library(library_path).expect("Error loading library") };

    // 라이브러리를 초기화합니다.
    let init_str = CString::new("CSUSER123455").unwrap();
    let home_dir = CString::new("C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\Dat").unwrap();
    let initialized = unsafe { eztrans_lib.initialize(&init_str.as_c_str(), &home_dir.as_c_str()) };

    if initialized {
        // 라이브러리 내의 함수를 호출하여 번역합니다.
        //let want_text: &str = "雨が降る";
        //let text_length = want_text.chars().count();
        /*
        let text_big: &str = {"職業紹介、騎士。
        近接職の派生職、近接の中で射程のある槍を使えます。
        他の近接職よりも魔術耐性が高く、全体的にそつのないステータスをしています。
        逆に言えば特化していない器用貧乏になりがちですが、二番手に配置とかするとしっかりと仕事をしてくれます。"};
        */
        let text_big: &str = {
            "プログラマーとデザイナー二人でUnityを使ったゲームを開発中。
        ゲーム開発している経過等をツイートします。
        アップする画像は制作中のものだったりするので色々変わります。"
        };
        let text_length = text_big.chars().count();
        //enhd 적용됐는지 확인용 - "蜜ドル辞典"
        let text: String = String::from(text_big);

        let translated_string: *const u16 = unsafe { eztrans_lib.translate(text) }; // 포인터로 변환하는데, 중요한 점은 길이가 바뀌지 않았다는 거. 길이 계산할 필요 없이. text_length를 쓰면 됨.

        // 문자열을 &str로 변환합니다..
        //let as_bytes = translated_string;
        //let as_bytes: &[u8] = translated_string.as_bytes();
        println!("Translated string: {:?}", translated_string);

        let mut len = 0;

        while unsafe { *translated_string.add(len) } != 0 {
            len += 1;
        }
        //println!("len: {}", len);//138
        //println!("text_length: {}", text_length);//101
        // *mut u16 포인터를 &[u16] 슬라이스로 변환
        let u16_slice = unsafe { slice::from_raw_parts(translated_string, text_length) };
        println!("u16_slice: {:?}", u16_slice);
        // UTF-16 슬라이스를 UTF-8으로 디코딩하여 &str을 얻습니다.
        let os_string = OsString::from_wide(u16_slice);
        let strrr = os_string
            .into_string()
            .expect("Failed to convert to string");

        println!("String: {}", strrr);

        // 라이브러리를 종료합니다.
        unsafe { eztrans_lib.terminate() };
    } else {
        println!("Failed to initialize EzTransLib");
    }
}

fn to_cp949_string(bytes: &[u8]) -> Result<String, String> {
    WINDOWS_949
        .decode(bytes, DecoderTrap::Replace)
        .map_err(|_| "Error decoding string".to_string())
}

fn to_hex_string(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{:02X}", byte)).collect()
}
