use anyhow::{Context, Result};

fn memory_map_executable_code(code: &Vec<u8>) -> Result<extern "C" fn() -> u64> {
    let func: extern "C" fn() -> u64 = unsafe {
        // 1. mmap to map read/write anonymous memory of size code
        let ptr = libc::mmap(
            0 as *mut libc::c_void,
            code.len(),
            libc::PROT_READ | libc::PROT_WRITE,
            libc::MAP_ANON | libc::MAP_PRIVATE,
            -1,
            0,
        );
        if ptr == libc::MAP_FAILED {
            return Err(std::io::Error::last_os_error())
                .context("memory mapping region for executable code");
        }
        // 2. copy code to memory
        std::ptr::copy_nonoverlapping(code.as_ptr(), ptr as *mut u8, code.len());
        // 3. mprotect the memory to read/exec
        let result = libc::mprotect(ptr, code.len(), libc::PROT_EXEC | libc::PROT_READ);
        if result == -1 {
            return Err(std::io::Error::last_os_error())
                .context("making memory mapped region executable");
        }
        // 4. reinterpret_cast memory pointer to function signature
        std::mem::transmute(ptr)
    };

    Ok(func)
}

fn main() -> Result<()> {
    #[rustfmt::skip]
    let code: Vec<u8> = vec![
        // mov qword rax, 0x42
        0xB8, 0x42, 0x00, 0x00, 0x00,
        // ret
        0xc3,
    ];

    let func = memory_map_executable_code(&code)?;
    let result = func();
    println!("Dynamic code returned 0x{:x}", result);
    Ok(())
}
