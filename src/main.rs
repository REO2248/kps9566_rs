use kps9566_rs::Kps9566Codec;
use std::env;

fn show_help(program_name: &str) {
    println!("국규 9566 복호화기 및 부호화기");
    println!("사용법:");
    println!("  {} <입력화일이름>                                    국규 9566 본문화일을 현시합니다.", program_name);
    println!("  {} <입력화일이름> <출력화일이름>                     국규 9566 본문화일을 UTF-8로 변환합니다.", program_name);
    println!("  {} -e, --encode <본문> <출력화일이름>                본문을 국규 9566으로 변환합니다.", program_name);
    println!("  {} -ef, --encode-file <입력화일이름> <출력화일이름>  UTF-8 본문화일을 국규 9566으로 변환합니다.", program_name);
    println!("  {} ,-h --help                                        도움말을 현시합니다.", program_name);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // 프로그람이름에서 경로를 제거
    let program_name = args[0]
        .split('\\')
        .last()
        .unwrap_or(&args[0])
        .split('/')
        .last()
        .unwrap_or(&args[0]);
    
    // 도움말현시
    if args.len() >= 2 && (args[1] == "--help" || args[1] == "-h") {
        show_help(program_name);
        return Ok(());
    }
    
    if args.len() < 2 {
        show_help(program_name);
        return Ok(());
    }

    let codec = Kps9566Codec::new()?;

    // 화일에서 부호화방식
    if args.len() >= 4 && (args[1] == "--encode-file" || args[1] == "-ef") {
        let input_file = &args[2];
        let output_file = &args[3];

        // 입력화일읽기
        let text = std::fs::read_to_string(input_file)?;
        
        // 부호화
        let encoded_bytes = codec.encode(&text);
        std::fs::write(output_file, &encoded_bytes)?;
        
        return Ok(());
    }

    // 본문 부호화방식
    if args.len() >= 4 && (args[1] == "--encode" || args[1] == "-e") {
        let text = &args[2];
        let output_file = &args[3];

        // 부호화
        let encoded_bytes = codec.encode(text);
        std::fs::write(output_file, &encoded_bytes)?;
        
        return Ok(());
    }

    // 복호화방식
    let input_file = &args[1];
    
    match std::fs::read(input_file) {
        Ok(file_data) => {
            match codec.decode(&file_data) {
                Ok(decoded_text) => {
                    if args.len() >= 3 {
                        let output_file = &args[2];
                        std::fs::write(output_file, &decoded_text)?;
                    } else {
                        println!("{}", decoded_text);
                    }
                }
                Err(e) => {
                    println!("오유가 발생하였습니다: {}", e);
                }
            }
        }
        Err(e) => {
            println!("화일읽기오유: {}", e);
        }
    }

    Ok(())
}
