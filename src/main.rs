#![feature(lang_items, core_intrinsics, rustc_private)]
#![feature(start)]
#![no_std]
#![no_main]

use core::intrinsics;
use core::panic::PanicInfo;

mod kernel_state;
mod video_memory_print;

use crate::video_memory_print::color::Color;
use crate::video_memory_print::print::{clear_screen, print_str};
use kernel_state::KernelState;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kernel_main()
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut kernel_state: KernelState = KernelState::init();
    clear_screen(&mut kernel_state);
    kernel_state.print_color.fg_color = Color::Green;
    print_str(
        "Welcome to VOS, an operating system developed in rust by Vidde!",
        &mut kernel_state,
    );
    loop {}
}

// These functions are used by the compiler, but not
// for a bare-bones hello world. These are normally
// provided by libstd.
#[lang = "eh_personality"]
#[no_mangle]
pub extern "C" fn rust_eh_personality() {}

#[lang = "panic_impl"]
#[no_mangle]
#[panic_handler]
pub extern "C" fn rust_begin_panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}
