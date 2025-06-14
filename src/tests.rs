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
        let mut window = Window::init(1280, 720, "Test 0")
            .unwrap();

        while !window.should_close() {
            let mut d = window.begin_drawing();
            d.clear_background(Color::BLACK);
        }
    }
}
