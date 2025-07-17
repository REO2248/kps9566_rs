# KPS 9566 Rs

조선민주주의인민공화국 국규 9566 문자부호화방식을 위한 Rust서고입니다.
인공지능도구를 사용하여 작성되였습니다.

## 개요

KPS 9566은 조선민주주의인민공화국에서 사용하는 문자부호입니다. 이 서고는 KPS 9566 부호와 유니코드 사이의 변환기능을 제공합니다.

## 기능

- KPS 9566 부호를 유니코드로 복호화
- 유니코드 본문을 KPS 9566 부호로 부호화  
- 명령 `kps9566convert`

## 설치

Rust와 Cargo가 설치되여있어야 합니다.

```bash
git clone https://github.com/REO2248/kps9566_rs.git
cd kps9566_rs
cargo build --release
```

## 사용법

### 서고로 사용

```rust
use kps9566_rs::Kps9566Codec;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let codec = Kps9566Codec::new()?;
    
    // 부호화
    let text = "안녕하십니까?";
    let encoded = codec.encode(text);
    
    // 복호화
    let decoded = codec.decode(&encoded)?;
    println!("{}", decoded);
    
    Ok(())
}
```

### 명령행도구로 사용

```bash
# 도움말현시
kps9566convert --help

# KPS 9566 화일을 현시
kps9566convert input.kps

# KPS 9566 화일을 UTF-8로 변환
kps9566convert input.kps output.txt

# 본문을 KPS 9566으로 부호화
kps9566convert --encode "안녕하세요" output.kps

# UTF-8 화일을 KPS 9566으로 변환
kps9566convert --encode-file input.txt output.kps
```

## 실현상세

이 서고는 다음과 같은 기능들을 제공합니다:

- `Kps9566Decoder`: KPS 9566 바이트렬을 유니코드 문자렬로 변환
- `Kps9566Encoder`: 유니코드 문자렬을 KPS 9566 바이트렬로 변환  
- `Kps9566Codec`: 부호화와 복호화기능을 모두 제공하는 통합형태

KPS 9566 문자대조표는 컴파일시간에 `build.rs`에 의해 `generate_mapping.py` 대본을 실행하여 생성됩니다.

## 개발

### 전제조건

- Rust 1.60 이상
- Python 3.6 이상 (문자대조표생성용)

### 구축

```bash
cargo build
```

### 시험

```bash
cargo test
```

### 실례실행

```bash
cargo run --example basic_usage
```
