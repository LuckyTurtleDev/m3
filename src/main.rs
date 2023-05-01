//! Folgende Zeiten wurden durch die eine 120-fps-Kamera erhoben:
//! 	- Zeit zwischen Neopixel und ersten grünen Pixel auf dem Display: 1 Frame = 8 ms
//! 	- Zeit zwischen Neopixel und einem komplett grünen Display: 7 Frames = 58 ms
//!
//! Die Zeit, welche von im Microcontroller eingebauten Timer erhoben wurden, beträgt 40 ms

#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    prelude::*,
    text::Text,
};
use heapless::String;
use pybadge::time::uptime;
use pybadge::Display;
use pybadge::NeoPixelColor;
use pybadge::{prelude::*, Color, PyBadge};
use pybadge_high as pybadge;

const NUM_LEDS: u8 = 5;

fn draw(display: &mut Display, time: u32) {
    let mut string = String::<32>::new();
    let style = MonoTextStyle::new(&FONT_6X10, Color::BLACK);
    string.clear();
    write!(string, "Zeit: {} ms", time).unwrap();
    display.clear(Color::GREEN).unwrap();
    Text::new(&string, Point::new(20, 30), style)
        .draw(display)
        .ok();
}

#[entry]
fn main() -> ! {
    let mut pybadge = PyBadge::take().unwrap();
    let mut display = pybadge.display;
    let mut buttons = pybadge.buttons;
    let mut timestamp;
    let mut button_a_pressed = false;

    loop {
        while button_a_pressed == buttons.a_pressed() {
            buttons.update();
        }

        if buttons.a_pressed() {
            pybadge
                .neopixel
                .write((0..NUM_LEDS).map(|_i| NeoPixelColor { r: 0, g: 2, b: 0 }))
                .unwrap();
            timestamp = uptime();
            display.clear(Color::GREEN).unwrap();
            let duration = uptime().0 - timestamp.0;
            draw(&mut display, duration)
        } else {
            pybadge
                .neopixel
                .write((0..NUM_LEDS).map(|_i| NeoPixelColor { r: 0, g: 0, b: 0 }))
                .unwrap();
            display.clear(Color::BLACK).unwrap();
        }
        button_a_pressed = !button_a_pressed;
    }
}