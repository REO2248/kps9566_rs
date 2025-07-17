use std::collections::HashMap;

mod mapping;
use mapping::create_kps9566_mapping;

/// KPS 9566 복호화기
pub struct Kps9566Decoder {
    mapping: HashMap<u16, char>,
}

/// KPS 9566 부호화기
pub struct Kps9566Encoder {
    reverse_mapping: HashMap<char, u16>,
}

/// KPS 9566 부호화기 및 복호화기
pub struct Kps9566Codec {
    decoder: Kps9566Decoder,
    encoder: Kps9566Encoder,
}

impl Kps9566Decoder {
    /// 새로 복호화기를 만들다
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mapping = create_kps9566_mapping();
        Ok(Self { mapping })
    }

    /// 바이트렬을 KPS 9566부호로 문자렬로 복호화
    pub fn decode(&self, bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        let mut result = String::new();
        let mut i = 0;

        while i < bytes.len() {
            let byte = bytes[i];

            // ASCII범위（0x00-0x7F
            if byte <= 0x7F {
                result.push(byte as char);
                i += 1;
            } else {
                // 2바이트문자로 처리
                if i + 1 < bytes.len() {
                    let high_byte = byte as u16;
                    let low_byte = bytes[i + 1] as u16;
                    let code = (high_byte << 8) | low_byte;

                    if let Some(&character) = self.mapping.get(&code) {
                        result.push(character);
                    } else {
                        // 대응이 없는 경우
                        result.push('�');
                    }
                    i += 2;
                } else {
                    // 불완전한 바이트렬인 경우
                    result.push('�');
                    i += 1;
                }
            }
        }

        Ok(result)
    }

    /// 화일에서 KPS 9566으로 부호화된 본문을 읽다
    pub fn decode_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<String, Box<dyn std::error::Error>> {
        let bytes = std::fs::read(path)?;
        self.decode(&bytes)
    }
}

impl Default for Kps9566Decoder {
    fn default() -> Self {
        Self::new().expect("KPS 9566복호화기의 초기화에 실패하였습니다")
    }
}

impl Kps9566Encoder {
    /// 새로 부호화기를 만들다
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let mapping = create_kps9566_mapping();

        let reverse_mapping: HashMap<char, u16> = mapping.iter()
            .map(|(&code, &ch)| (ch, code))
            .collect();
        
        Ok(Self { reverse_mapping })
    }

    ///문자렬을 KPS 9566으로 부호화해서 바이트렬로 변환
    pub fn encode(&self, text: &str) -> Vec<u8> {
        let mut result = Vec::new();

        for ch in text.chars() {
            // ASCII범위의 문자는 그대로 처리
            if (ch as u32) <= 0x7F {
                result.push(ch as u8);
            } else {
                if let Some(&code) = self.reverse_mapping.get(&ch) {
                    result.push((code >> 8) as u8);  // 상위바이트
                    result.push((code & 0xFF) as u8); // 하위바이트
                } else {
                    // 대응이 없는 경우
                    if let Some(&replacement_code) = self.reverse_mapping.get(&'�') {
                        result.push((replacement_code >> 8) as u8);
                        result.push((replacement_code & 0xFF) as u8);
                    } else {
                        // �도 매핑에 없는 경우는 ?（ASCII）를 사용
                        result.push(b'?');
                    }
                }
            }
        }

        result
    }

    /// 문자렬을 KPS 9566으로 화일에 쓰다
    pub fn encode_to_file<P: AsRef<std::path::Path>>(&self, text: &str, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.encode(text);
        std::fs::write(path, bytes)?;
        Ok(())
    }
}

impl Default for Kps9566Encoder {
    fn default() -> Self {
        Self::new().expect("KPS 9566부호화기의 초기화에 실패하였습니다")
    }
}

impl Kps9566Codec {
    /// 새로 만들다
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let decoder = Kps9566Decoder::new()?;
        let encoder = Kps9566Encoder::new()?;
        Ok(Self { decoder, encoder })
    }

    /// 바이트렬을 KPS 9566으로 부호화된 문자렬로 복호화
    pub fn decode(&self, bytes: &[u8]) -> Result<String, Box<dyn std::error::Error>> {
        self.decoder.decode(bytes)
    }

    /// 문자렬을 KPS 9566으로 부호화해서 바이트렬로 변환
    pub fn encode(&self, text: &str) -> Vec<u8> {
        self.encoder.encode(text)
    }

    /// 화일에서 KPS 9566으로 부호화된 본문을 읽다
    pub fn decode_file<P: AsRef<std::path::Path>>(&self, path: P) -> Result<String, Box<dyn std::error::Error>> {
        self.decoder.decode_file(path)
    }

    /// 문자렬을 KPS 9566으로 부호화해서 화일에 쓴다
    pub fn encode_to_file<P: AsRef<std::path::Path>>(&self, text: &str, path: P) -> Result<(), Box<dyn std::error::Error>> {
        self.encoder.encode_to_file(text, path)
    }
}

impl Default for Kps9566Codec {
    fn default() -> Self {
        Self::new().expect("KPS 9566처리기의 초기화에 실패하였습니다")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_decode() {
        let decoder = Kps9566Decoder::new().unwrap();
        let ascii_bytes = b"Hello, World!";
        let result = decoder.decode(ascii_bytes).unwrap();
        assert_eq!(result, "Hello, World!");
    }

    #[test]
    fn test_ascii_encode() {
        let encoder = Kps9566Encoder::new().unwrap();
        let result = encoder.encode("Hello, World!");
        assert_eq!(result, b"Hello, World!");
    }

    #[test]
    fn test_mixed_content() {
        let decoder = Kps9566Decoder::new().unwrap();
        
        // ASCII + KPS 956 시험
        let test_bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x81, 0x41]; // "Hello" + 0x8141
        let result = decoder.decode(&test_bytes).unwrap();

        // 처음 5문자는 ASCII
        assert_eq!(&result[..5], "Hello");

        // 마지막 문자는 KPS 9566
        if decoder.mapping.contains_key(&0x8141) {
            assert_eq!(result.chars().count(), 6);
        }
    }

    #[test]
    fn test_codec_roundtrip() {
        let codec = Kps9566Codec::new().unwrap();
        let original_text = "Hello, World!";

        // 부호화  → 복호화 시험
        let encoded = codec.encode(original_text);
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(original_text, decoded);
    }

    #[test]
    fn test_codec_mixed_content() {
        let codec = Kps9566Codec::new().unwrap();
        
        // ASCII + KPS 9566 시험
        let test_bytes = vec![0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x81, 0x41]; // "Hello" + 0x8141
        let result = codec.decode(&test_bytes).unwrap();

        // 처음 5문자는 ASCII
        assert_eq!(&result[..5], "Hello");
        assert_eq!(result.chars().count(), 6);
    }

    #[test]
    fn test_korean_encoding() {
        let codec = Kps9566Codec::new().unwrap();
        
        // 조선어본문 시험
        let korean_text = "안녕";
        println!("시험본문: {}", korean_text);
        
        let encoded = codec.encode(korean_text);
        println!("부호화성공: {:02X?}", encoded);
        
        let decoded = codec.decode(&encoded).unwrap();
        println!("복호화결과: {}", decoded);
        assert_eq!(korean_text, decoded);
    }

    #[test]
    fn test_mixed_korean_ascii() {
        let codec = Kps9566Codec::new().unwrap();
        
        // ASCII와 KPS 9566이 섞인 시험
        let mixed_text = "Hello 세계";
        
        let encoded = codec.encode(mixed_text);
        let decoded = codec.decode(&encoded).unwrap();
        assert_eq!(mixed_text, decoded);
    }

    #[test]
    fn test_long_korean_text() {
        let codec = Kps9566Codec::new().unwrap();
        
        // 긴 조선어본문
        let long_text = "안녕하십니까? 공격전이다";
        println!("시험본문: {}", long_text);
        
        // 각 문자가 매핑에 있는지 확인
        for (i, ch) in long_text.chars().enumerate() {
            let code_point = ch as u32;
            if code_point <= 0x7F {
                println!("  [{}] '{}' - ASCII (0x{:02X})", i, ch, code_point);
            } else {
                // KPS 9566에서 해당 문자를 찾아봄
                let found = codec.encoder.reverse_mapping.contains_key(&ch);
                println!("  [{}] '{}' - 조선어 (U+{:04X}) - 존재: {}", i, ch, code_point, found);
            }
        }
        
        let encoded = codec.encode(long_text);
        println!("전체본문부호화성공!");
        println!("부호화된 바이트수: {}", encoded.len());
        
        let decoded = codec.decode(&encoded).unwrap();
        println!("복호화결과: {}", decoded);
        assert_eq!(long_text, decoded);
    }

    #[test]
    fn test_unsupported_characters() {
        let codec = Kps9566Codec::new().unwrap();
        
        // 지원하지 않는 문자 시험
        let text_with_unsupported = "Hello 🌟 World";  // 그림문자는 지원하지 않음
        
        let encoded = codec.encode(text_with_unsupported);
        let decoded = codec.decode(&encoded).unwrap();

        // 🌟가 � 또는 ?에 대체되였을 것이다.
        assert!(decoded.contains("Hello"));
        assert!(decoded.contains("World"));
        println!("원천본문: {}", text_with_unsupported);
        println!("복호화결과: {}", decoded);
    }
}
