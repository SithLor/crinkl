use stdx::prelude::*;

use std::slice;
fn execute_shellcode(shellcode: &[u8]) {
    unsafe {
        // Allocate executable memory
        let exec_mem = libc::mmap(
            std::ptr::null_mut(),
            shellcode.len(),
            libc::PROT_READ | libc::PROT_WRITE | libc::PROT_EXEC,
            libc::MAP_PRIVATE | libc::MAP_ANONYMOUS,
            -1,
            0,
        );

        if exec_mem == libc::MAP_FAILED {
            panic!("mmap failed");
        }

        // Copy shellcode to executable memory
        std::ptr::copy_nonoverlapping(shellcode.as_ptr(), exec_mem as *mut u8, shellcode.len());

        // Define a function pointer to the shellcode
        let shellcode_fn: extern "C" fn() = std::mem::transmute(exec_mem);

        // Execute the shellcode
        shellcode_fn();

        // Free the allocated memory
        libc::munmap(exec_mem, shellcode.len());
    }
}

fn main() {
    // Example shellcode: this is a no-op (infinite loop)
    let shellcode: [u8; 1] = [0x90]; // NOP instruction
    
    
    let game_json = "{
        \"game\": {
            \"title\": \"Super Mario Bros.\",
            \"platform\": \"NES\",
            \"year\": 1985
        }
    }";
    let checksum = game_json.crc128_encode();

    println!("CRC128 checksum 1: {}", checksum);
    let game_json_modified = "{
        \"game\": {
            \"title\": \"Supeer Mario Bros.\",
            \"platform\": \"NES\",
            \"year\": 1985
        }
    }";
    let checksum_1: u128 = game_json_modified.crc128_encode();

    println!("CRC128 checksum 2: {}", checksum);

    //check if the checksum is correct
    let c = game_json.crc128_check(checksum_1);

    println!("CRC128 checksum 1: {}", c);

    //execute_shellcode(&shellcode);
}