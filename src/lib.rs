#[cfg(feature = "buddy-alloc")]
mod alloc;
mod colors;
mod controls;
mod player;
mod wasm4;
use colors::*;
use controls::*;
use noise::{NoiseFn, Perlin, Seedable};
use player::Player;
use wasm4::*;

fn write_disk<T>(src: *const T, size: u32) {
    unsafe { std::ptr::copy(src, 0x19a0 as *mut T, size as usize) };
}

fn read_disk<T>(des: *mut T, size: u32) {
    unsafe { std::ptr::copy(0x19a0 as *mut T, des, size as usize) };
}

static mut PREV_MOUSE: Mouse = Mouse {
    x: 0,
    y: 0,
    buttons: 0,
};
fn update_previous() {
    unsafe {
        PREV_MOUSE = Mouse::new();
    }
}

fn clone_previousMouse() -> Mouse {
    unsafe { return PREV_MOUSE.clone() }
}

fn set_draw_color(color: u16) {
    unsafe {
        *DRAW_COLORS = color.into();
    }
}
#[no_mangle]
fn update() {
    unsafe { *SYSTEM_FLAGS |= SYSTEM_PRESERVE_FRAMEBUFFER };
    let perlin = Perlin::new().set_seed(0);
    // for i in (-10..10).map(|x| x as f64 * 0.01) {
    //     trace(format!("{:?}", perlin.get([i, i])));
    // }

    let mouse = Mouse::new();
    let game_pad = GamePad::new(0);
    let p_mouse = clone_previousMouse();
    Player::update(&game_pad);
    for x in (0..160).map(|x_pos| x_pos as f64 * 0.1) {
        for y in (0..160).map(|y_| y_ as f64 * 0.1) {
            let perlin_at_point = perlin.get([x + Player::get_x(), y + Player::get_y()]);
            // trace(format!("{}", perlin_at_point));
            if (perlin_at_point >= -1.0 && perlin_at_point < -0.5) {
                set_draw_color(0x1);
            } else if (perlin_at_point >= -0.5 && perlin_at_point < 0.0) {
                set_draw_color(0x2);
            } else if (perlin_at_point >= 0.0 && perlin_at_point < 0.5) {
                set_draw_color(0x3);
            } else if (perlin_at_point >= 0.5 && perlin_at_point <= 1.0) {
                set_draw_color(0x4);
            } else {
                panic!("thingthign");
            }
            // trace(format!("{}, {}", x * 10.0, y * 10.0));
            pixel((x * 10.0) as i32, (y * 10.0) as i32);
        }
    }
    update_previous();
    // blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    // text("Press X to blink", 16, 90);
}
