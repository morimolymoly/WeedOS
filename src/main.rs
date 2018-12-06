#![no_std]
#![no_main]
// println!マクロはつかえないこれはstdライブラリの範疇(stdoutはOSが用意するよね)
/*
fn main() {
}*/

// mac限定
// cargo rustc -- -Z pre-link-arg=-lSystem
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}

use core::panic::PanicInfo;

// panicハンドラはpanicが起こったときにrustが呼び出す
// 返り値の!は終わらない関数であることを意味する
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
