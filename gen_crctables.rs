use std::fs::File;
use std::io::{self, Write};

const CRC16_POLYNOMIAL: u16 = 0xA001;
const CRC32_POLYNOMIAL: u32 = 0xEDB88320;
const CRC64_POLYNOMIAL: u64 = 0x42F0E1EBA9EA3693;
const CRC128_POLYNOMIAL: u128 = 0x42F0E1EBA9EA369342F0E1EBA9EA3693;

fn generate_crc16_table() -> [u16; 256] {
    let mut table = [0u16; 256];
    for i in 0..256 {
        let mut crc = i as u16;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC16_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
        table[i] = crc;
    }
    table
}

fn generate_crc32_table() -> [u32; 256] {
    let mut table = [0u32; 256];
    for i in 0..256 {
        let mut crc = i as u32;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC32_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
        table[i] = crc;
    }
    table
}

fn generate_crc64_table() -> [u64; 256] {
    let mut table = [0u64; 256];
    for i in 0..256 {
        let mut crc = i as u64;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC64_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
        table[i] = crc;
    }
    table
}

fn generate_crc128_table() -> [u128; 256] {
    let mut table = [0u128; 256];
    for i in 0..256 {
        let mut crc = i as u128;
        for _ in 0..8 {
            if crc & 1 != 0 {
                crc = (crc >> 1) ^ CRC128_POLYNOMIAL;
            } else {
                crc >>= 1;
            }
        }
        table[i] = crc;
    }
    table
}

fn write_table_to_file<T: std::fmt::UpperHex>(file: &mut File, table: &[T], name: &str) -> io::Result<()> {
    writeln!(file, "pub const {}: [{}; 256] = [", name, std::any::type_name::<T>())?;
    for i in 0..256 {
        if i % 4 == 0 {
            write!(file, "    ")?;
        }
        write!(file, "0x{:X}, ", table[i])?;
        if i % 4 == 3 {
            writeln!(file)?;
        }
    }
    writeln!(file, "];")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let crc16_table = generate_crc16_table();
    let crc32_table = generate_crc32_table();
    let crc64_table = generate_crc64_table();
    let crc128_table = generate_crc128_table();

    let mut file = File::create("crc_tables.rs")?;

    write_table_to_file(&mut file, &crc16_table, "CRC16_TABLE")?;
    write_table_to_file(&mut file, &crc32_table, "CRC32_TABLE")?;
    write_table_to_file(&mut file, &crc64_table, "CRC64_TABLE")?;
    write_table_to_file(&mut file, &crc128_table, "CRC128_TABLE")?;

    Ok(())
}