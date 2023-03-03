#![allow(dead_code, unused_variables)]

use mod_draw_lib::core::mod_color::color;
use mod_draw_lib::core::mod_shapes::shapes;
use rgb::*;

//https://doc.rust-lang.org/cargo/reference/features.html

fn main() {
    println!("Hello World");
    let px = RGB {
        r: 255_u16,
        g: 0_u16,
        b: 255_u16,
    };
    let width = 10;
    let height = 20;

    let rc = shapes::Rectangle {
        color: px,
        width,
        height,
    };

    color::draw_line(
        rc.width.try_into().unwrap(),
        rc.height.try_into().unwrap(),
        &rc.color,
    );
}
