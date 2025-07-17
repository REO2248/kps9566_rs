use std::fs;
use std::path::Path;
use std::process::Command;

fn main() {
    // 콤파일시에 목록화일을 자동생성
    let mapping_file = Path::new("src/mapping.rs");
    let kps_file = Path::new("KPS_9566.txt");
    let python_script = Path::new("generate_mapping.py");

    // KPS_9566.txt화일이 존재하는지 확인
    if !kps_file.exists() {
        panic!("KPS_9566.txt화일을 찾을수 없습니다.");
    }

    // Python대본이 존재하는지 확인
    if !python_script.exists() {
        panic!("generate_mapping.py대본을 찾을수 없습니다.");
    }

    // 목록화일이 존재하지 않거나 KPS_9566.txt보다 오래된 경우 재생성
    let shouldgenerate = if mapping_file.exists() {
        let kps_modified = fs::metadata(kps_file).unwrap().modified().unwrap();
        let mapping_modified = fs::metadata(mapping_file).unwrap().modified().unwrap();
        kps_modified > mapping_modified
    } else {
        true
    };

    if shouldgenerate {
        println!("cargo:warning=목록화일을 생성중...");

        // Python대본을 실행
        let output = Command::new("python")
            .arg("generate_mapping.py")
            .arg("KPS_9566.txt")
            .arg("src/mapping.rs")
            .output()
            .expect("Python대본의 실행에 실패했습니다. Python이 설치되여 있는지 확인하십시오.");

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            panic!("목록화일생성오유: {}", stderr);
        }

        println!("cargo:warning=목록화일이 성과적으로 생성되였습니다");
    }

    // cargo에 의존성 알리기
    println!("cargo:rerun-if-changed=KPS_9566.txt");
    println!("cargo:rerun-if-changed=generate_mapping.py");
    println!("cargo:rerun-if-changed=build.rs");
}
