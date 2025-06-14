#![cfg(test)]

use std::sync::{Mutex, MutexGuard};
use crate::prelude::*;

/// Exclusive access to Raylib window
fn await_turn() -> MutexGuard<'static, ()> {
    static TALKING_STICK: Mutex<()> = Mutex::new(());

    TALKING_STICK
        .lock()
        .unwrap_or_else(|x| {
            TALKING_STICK.clear_poison();
            x.into_inner()
        })
}

#[test]
fn test0() {
    let _turn = await_turn();
    {
        let mut rl = Window::init(1280, 720, "Test 0")
            .unwrap();

        while !rl.should_close() {
            rl.draw(|rl, d, m| {
                d.clear_background(Color::BLACK);
                let s = c"Hello world!";
                let font_size = 10;
                let width = rl.measure_text(s, font_size);
                let mut m = m.begin_scissor(10, 5, 20, 8);
                d.draw_rectangle(5, 5, width, font_size as i32, Color::GREEN);
                d.draw_text(s, 5, 5, font_size, Color::BLUE);
            });
        }
    }
}

#[test]
fn test1() {
    use crate::low_level::text::TextToSnakeHandle;
    let mut handle = TextToSnakeHandle::get().unwrap();
    let result = handle.text_to_snake(c"HelloWorld");
    assert_eq!(result, c"hello_world");
    assert_eq!(handle.text_to_snake(c"AppleOrangeBanana"), c"apple_orange_banana");
    // assert_eq!(result, c"hello_world");
}
