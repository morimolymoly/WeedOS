#![no_std]
#![no_main]

// println!マクロはつかえないこれはstdライブラリの範疇(stdoutはOSが用意するよね)
/*
fn main() {
}*/

static HELLO: &[u8] = b"Hello, World!";

// LLVMによりLinuxっぽい規則で
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

use core::panic::PanicInfo;

// panicハンドラはpanicが起こったときにrustが呼び出す
// 返り値の!は終わらない関数であることを意味する
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
