mod eztrans_lib;
// `eztrans_lib`는 실제 라이브러리 크레이트 이름입니다.
use std::ffi::CString;
use std::fs;

fn main() {
    // 동적 라이브러리의 경로를 지정합니다.
    let library_path = "C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\J2KEngine.dll";

    // 동적 라이브러리를 로드하고 EzTransLib 인스턴스를 가져옵니다.
    let eztrans_lib = unsafe {
        eztrans_lib::load_library(library_path)
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
    //enhd 적용됐는지 확인용 - "蜜ドル辞典"
    //let text: String = String::from(text_big);
    let text = read_text_file("test.txt");
    let text_2 = eztrans_lib::jp_str_enum::string(&text);
    let translated_string = unsafe { eztrans_lib.translate(text_2) };

    // 문자열을 &str로 변환합니다..
    println!("Translated string: {}", translated_string);

    // 라이브러리를 종료합니다.
    unsafe { eztrans_lib.terminate() };
}

fn read_text_file(path: &str) -> String {
    let contents = fs::read_to_string(path).expect("Something wrong reading the file");
    contents
}
