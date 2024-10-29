const CRC32_POLYNOMIAL: u32 = 0xEDB88320;

fn compute_crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFFFFFF;
    for &byte in bytes {
        crc ^= byte as u32;
        for _ in 0..8 {
            let mask = if crc & 1 != 0 { CRC32_POLYNOMIAL } else { 0 };
            crc = (crc >> 1) ^ mask;
        }
    }
    !crc
}

trait CrcEncode {
    fn crc_encode(&self) -> u32;
}

impl CrcEncode for str {
    fn crc_encode(&self) -> u32 {
        compute_crc32(self.as_bytes())
    }
}

impl CrcEncode for Vec<u8> {
    fn crc_encode(&self) -> u32 {
        compute_crc32(self)
    }
}

trait CrcCheck {
    fn crc_check(&self, checksum: u32) -> bool;
}

impl CrcCheck for str {
    fn crc_check(&self, checksum: u32) -> bool {
        self.crc_encode() == checksum
    }
}

impl CrcCheck for Vec<u8> {
    fn crc_check(&self, checksum: u32) -> bool {
        self.crc_encode() == checksum
    }
}

// Usage examples:
// "t".crc_encode();
// let encoded = "t".crc_encode();
// let vec = vec![1, 2, 3, 4, 5];
// let encoded_vec = vec.crc_encode();
// let is_valid = "value sum".crc_check(encoded);