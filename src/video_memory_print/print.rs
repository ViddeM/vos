use crate::kernel_state::KernelState;
use crate::video_memory_print::char::Char;
use crate::video_memory_print::char_color::CharColor;
use crate::video_memory_print::color::Color;

const NUM_COLS: u8 = 80;
const NUM_ROWS: u8 = 25;

fn write_char(char: &Char, row: u8, col: u8, state: &mut KernelState) {
    let index = col as usize + row as usize * NUM_COLS as usize;
    state.video_memory[index] = char.into();
}

pub fn print_str(a: &str, state: &mut KernelState) {
    let chars = a.as_bytes();
    for i in 0..chars.len() {
        let col = (i % NUM_COLS as usize) as u8;
        let row = ((i - col as usize) / NUM_COLS as usize) as u8;
        write_char(
            &Char {
                character: chars[i],
                color: state.print_color.into(),
            },
            row,
            col,
            state,
        )
    }
}

fn clear_row(row: u8, state: &mut KernelState) {
    let char = Char {
        character: b' ',
        color: CharColor {
            fg_color: Color::White,
            bg_color: Color::Black,
        }
        .into(),
    };
    for col in 0..NUM_COLS {
        write_char(&char, row, col, state)
    }
}

pub fn clear_screen(state: &mut KernelState) {
    for row in 0..NUM_ROWS {
        clear_row(row, state)
    }
}
