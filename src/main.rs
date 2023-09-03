use num_cpus;
use rayon::prelude::*;
use rayon::ThreadPoolBuilder;
use std::env;
use std::ffi::CString;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;
use std::time::Instant;
use Qootrans;

fn main() {
    // 동적 라이브러리의 경로를 지정합니다.
    let library_path = "C:\\Program Files (x86)\\ChangShinSoft\\ezTrans XP\\J2KEngine.dll";

    // 동적 라이브러리를 로드하고 EzTransLib 인스턴스를 가져옵니다.
    let eztrans_lib = unsafe {
        Qootrans::load_library(library_path)
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

    //enhd 적용됐는지 확인용 - "蜜ドル辞典"
    //test.txt

    //폴더나 파일 경로를 입력받습니다.
    let args: Vec<String> = env::args().collect();
    let args = if args.len() == 1 {
        println!("전달받은 인수가 없습니다. 경로를 입력해주세요.");
        let mut args = String::new();
        io::stdin()
            .read_line(&mut args)
            .expect("입력받지 못했습니다.");
        let args = args.trim().to_string();
        println!("전달받은 경로: {:?}", args);
        args
    } else {
        args[1].clone()
    };

    //파일을 읽습니다.
    let text = fs::read_to_string(&args).expect("Something wrong reading the file");

    //파일 크기를 구합니다.
    let file_size = match get_file_size(&args) {
        Ok(file_size) => file_size,
        Err(e) => {
            println!("Error getting file size: {:?}", e);
            e.raw_os_error().unwrap() as u64
        }
    };
    //let num_threads = num_cpus::get(); // 시스템의 CPU 코어 수를 가져옵니다. -. 코어 갯수만큼 프로세스가 만들어집니다.

    let now = Instant::now();

    //기존 방식.
    //이 코드를 각 프로세스에 집어넣고, 파일을 프로세스 수만큼 쪼개어 분배합니다.
    let results = eztrans_lib.translate(&text);

    // 문자열을 &str로 변환합니다..
    let processing_time = now.elapsed();
    println!("time: {:?}", processing_time);
    let kilobytes_size = file_size / 1024;
    println!("File size: {} kb", kilobytes_size);
    let processing_time = if processing_time.as_secs() == 0 {
        1
    } else {
        processing_time.as_secs()
    };
    let speed = kilobytes_size / processing_time;
    println!("Speed: {} kb/sec", speed);
    //println!("Results: {:?}", text_lines);

    //텍스트 파일을 저장합니다.
    let mut file = File::create("save.txt").expect("create failed");
    file.write_all(results.as_bytes()).expect("write failed");

    // 라이브러리를 종료합니다.
    unsafe { eztrans_lib.terminate() };
}

fn get_file_size(file_path: &str) -> Result<u64, std::io::Error> {
    let metadata = fs::metadata(file_path)?;

    // Metadata의 len() 메서드를 사용하여 파일 크기를 얻습니다.
    let file_size = metadata.len();
    Ok(file_size)
}

/*Todo
1. 파일들을 먼저 읽음.
2. 한 파일에 든 텍스트를 \n로 쪼개 멀티쓰레딩으로 번역.
3. 번역된 파일을 저장.
//해야할 일. 1. 만약 텍스트 파일이 한 줄로 길면 어떡함? - 온점을 기준으로 문자열을 나누고 다시 합치는 방식으로 해야할 듯.
//리빙포인트. enhd 및 정규식 적용은 한줄씩 처리해서 보내나, 한꺼번에 통으로 보내나 둘 다 적용된다.


*/

//1000줄 번역할 때,
//멀티쓰레드 코드 경우, 14초.
//단일쓰레드 코드 경우, 3초.
//왜 멀티쓰레드가 더 느릴까?
//-> 멀티쓰레드는 쓰레드를 생성하고, 쓰레드를 종료하는데 시간이 걸린다고 추측.

//텍스트 파일 4mb 번역할 때,
//단일 스레드인 경우 311초 -> 다시 말해 5분 11초.

//텍스트 파일 113kb 번역할 때,
//단일 스레드(한줄씩 처리)인 경우 29초.
//단일 스레드(전체 처리)인 경우 12초.

//enhd 및 정규식 적용은 한줄씩 처리해서 보내나, 한꺼번에 통으로 보내나 같은 결과물을 보낸다.
//한 줄씩 나눠서 보낼 필요가 사라졌다.
//파일 크기에 따라서 한꺼번에 보내는게 더 빠를 수도 있다.
//이 때문에 어느 정도의 크기로 데이터를 쪼개서 보내야할지가 관건이다. - chunk_size를 조절해 볼 것.

//par_iter_mut 방식(쓰레드는 자동) - 53.21초. - 병목현상 발생.
//par_iter_mut와 pool(4쓰레드) 방식 - 12.94초.
//par_iter 방식(쓰레드는 자동) - 52.53초. - 병목현상으로 매우 느리나, par_iter_mut보다는 빠름.
//par_iter와 pool(4쓰레드) 방식 - 12.64초.
//단일 쓰레드 방식 - 12.18초.

/*
1.단일 쓰레드의 최적 성능을 구할 예정.
2.멀티프로세스로 쓰레드를 나눌 예정.
파일 크기 / 처리하는 데 걸린 시간 = 처리하는 속도
test1 = 113kb / 12.95초 = 8.73kb/s
test2 = 226kb / 25.88초 = 8.73kb/s - 2스레드로 처리한 결과 32초. - 7.06kb/s
test3 = 4033kb / 327초 = 12.33kb/s - 2스레드로 처리한 결과 727초 - 5.54kb/s
    //스레드 풀 정의 코드.
    let pool = ThreadPoolBuilder::new()
        .num_threads(4)
        .build()
        .expect("Failed to build thread pool");



    let mut results: Vec<_> = text.lines().map(|line| line.to_owned()).collect();
    // 문자열 벡터를 병렬 반복 처리합니다. - par_iter_mut를 사용하는 방법.
    results.par_iter_mut().for_each(|result| {
        *result = unsafe { eztrans_lib.translate(&result) };
    });


    let mut results: Vec<_> = text.lines().map(|line| line.to_owned()).collect();
    pool.install(|| {
        results.par_iter_mut().for_each(|result| {
            *result = unsafe { eztrans_lib.translate(&result) };
        });
    });


    let text_lines: Vec<_> = text.lines().collect();
    // 문자열 벡터를 병렬 반복 처리합니다. - par_iter를 사용하는 방법.

    let results = text_lines.par_iter()
        .map(|line| {
            // 각 문자열에 대해 번역을 실행합니다.
            unsafe { eztrans_lib.translate(line) }
        }).collect::<Vec<_>>(); // 결과들을 벡터로 모읍니다.


    // 문자열 벡터를 병렬 반복 처리합니다. - par_iter를 사용하나 4쓰레드만 사용하는 방법.
    let text_lines: Vec<_> = text.lines().collect();
    let results = pool.install(|| {
        text_lines.par_iter()
        .map(|line| {
            // 각 문자열에 대해 번역을 실행합니다.
            unsafe { eztrans_lib.translate(line) }
        }).collect::<Vec<_>>() // 결과들을 벡터로 모읍니다.
    });















*/
