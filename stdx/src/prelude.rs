#[cfg(feature = "crc")]
pub use crate::crc::{
    CRC128_TABLE, CRC32_TABLE, CRC64_TABLE, CRC16_TABLE,
    CrcCheck, CrcEncode,
    compute_crc128, compute_crc16, compute_crc32, compute_crc64
};
