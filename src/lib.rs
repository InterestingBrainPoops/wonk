#[cfg(feature = "buddy-alloc")]
mod alloc;
mod colors;
mod controls;
mod player;
mod wasm4;
// use wasm_timer::Instant;

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

static mut cpos: (i32, i32) = (0, 0);

fn update_cpos() {
    unsafe {
        cpos.0 += 1;
        if (cpos.0 == SCREEN_SIZE) {
            cpos.1 += 1;
            cpos.0 = 0;
        }
        if (cpos.1 == SCREEN_SIZE) {
            cpos.1 = 0;
        }
    }
}

fn get_cpos() -> (i32, i32) {
    unsafe { cpos.clone() }
}

#[no_mangle]
fn update() {
    unsafe { *SYSTEM_FLAGS |= SYSTEM_PRESERVE_FRAMEBUFFER };
    let perlin = Perlin::new().set_seed(0);
    let perlin_layer_2 = Perlin::new().set_seed(100);
    // for i in (-10..10).map(|x| x as f64 * 0.01) {
    //     trace(format!("{:?}", perlin.get([i, i])));
    // }

    let mouse = Mouse::new();
    let game_pad = GamePad::new(0);
    let p_mouse = clone_previousMouse();
    //     let start = Instant::now();

    Player::update(&game_pad);
    for _ in 0..(160 * 160) {
        update_cpos();
        let c_pixel = get_cpos();
        let x = c_pixel.0;
        let y = c_pixel.1;
        let scale = 0.1;
        let perlin_at_point = perlin.get([
            x as f64 * scale + Player::get_x(),
            y as f64 * scale + Player::get_y(),
            Player::get_z(),
        ]) + perlin_layer_2.get([
            x as f64 * scale + Player::get_x(),
            y as f64 * scale + Player::get_y(),
            Player::get_z(),
        ]);
        // trace(format!("{}", perlin_at_point));
        if (perlin_at_point < -0.75) {
            set_draw_color(0x1);
        } else if (perlin_at_point >= -0.75 && perlin_at_point < 0.0) {
            set_draw_color(0x2);
        } else if (perlin_at_point >= 0.0 && perlin_at_point < 0.75) {
            set_draw_color(0x3);
        } else if (perlin_at_point >= 0.75) {
            set_draw_color(0x4);
        }
        // trace(format!("{}, {}", x * 10.0, y * 10.0));
        pixel((x) as i32, (y) as i32);
    }

    update_previous();
    // blit(&SMILEY, 76, 76, 8, 8, BLIT_1BPP);
    // text("Press X to blink", 16, 90);
}
