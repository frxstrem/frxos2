use core::ops::{Deref, DerefMut, Index, IndexMut};

use spin::{Mutex, MutexGuard};
use volatile::Volatile;

mod character;
pub use character::*;

pub struct VgaBuffer {
    guard: MutexGuard<'static, ()>,
    buf: &'static mut [Volatile<Char>],
    rows: usize,
    cols: usize,
}

impl Deref for VgaBuffer {
    type Target = [Volatile<Char>];
    fn deref(&self) -> &[Volatile<Char>] {
        self.buf
    }
}

impl DerefMut for VgaBuffer {
    fn deref_mut(&mut self) -> &mut [Volatile<Char>] {
        self.buf
    }
}

static BUFFER_LOCK: Mutex<()> = Mutex::new(());

pub fn buffer() -> VgaBuffer {
    let guard = BUFFER_LOCK.lock();
    
    
    let rows = 25;
    let cols = 80;
    let buf = 0xb8000 as *mut Volatile<Char>;

    let buf = unsafe {
        core::slice::from_raw_parts_mut(buf, rows * cols)
    };

    VgaBuffer {
        guard,
        buf,
        rows, cols,
    }
}

static WRITER_STATE: Mutex<VgaWriterState> = Mutex::new(VgaWriterState::new());

struct VgaWriterState {
    index: usize,
    color: ColorCode,
}

pub struct VgaWriter {
    state: MutexGuard<'static, VgaWriterState>,
    buffer: VgaBuffer,
}

impl VgaWriterState {
    const fn new() -> VgaWriterState {
        VgaWriterState {
            index: 0,
            color: ColorCode::new(Color::White, Color::Black, false),
        }
    }
}

impl VgaWriter {
    fn scroll_line(&mut self) {
        for (i, j) in (self.buffer.cols..).enumerate() {
            let b = self.buffer[j].read();
            self.buffer[i].write(b);
        }

        while self.state.index >= self.buffer.cols {
            self.state.index -= self.buffer.cols;
        }
    }
}

impl core::fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for ch in s.chars() {
            self.write_char(ch);
        }
        Ok(())
    }

    fn write_char(&mut self, ch: char) -> core::fmt::Result {
        let ch = match ch {
            '\n' => {
                let row_rest = self.buffer.cols - self.state.index % self.buffer.cols;
                self.state.index += row_rest;
                None
            }
            '\u{0020}'..='\u{007e}' => Some(ch as u8),
            _ => Some(0xfe),
        };

        let ch = ch.map(|ch| Char::new(ch, self.state.color));

        if self.state.index >= self.buffer.len() {
            self.scroll_line();
        }

        if let Some(ch) = ch {
            self.buffer[self.state.index].write(ch);
            self.state.index += 1;
        }

        Ok(())
    }
}

pub fn writer() -> VgaWriter {
    let state = WRITER_STATE.lock();
    let buffer = buffer();

    VgaWriter {
        state,
        buffer,
    }
}