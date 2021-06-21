use crate::video_memory_print::char_color::CharColor;
use crate::video_memory_print::color::Color;

const VIDEO_MEMORY_ADDRESS: u32 = 0xb8000;
const VIDEO_MEMORY_LEN: usize = 4000;

pub struct KernelState<'a> {
    pub video_memory: &'a mut [u16],
    pub print_color: CharColor,
}

impl KernelState<'_> {
    pub fn init() -> KernelState<'static> {
        KernelState {
            video_memory: unsafe {
                core::slice::from_raw_parts_mut(VIDEO_MEMORY_ADDRESS as *mut u16, VIDEO_MEMORY_LEN)
            },
            print_color: CharColor {
                bg_color: Color::Black,
                fg_color: Color::White,
            },
        }
    }
}
