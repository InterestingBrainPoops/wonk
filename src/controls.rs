use crate::wasm4::*;

#[derive(Clone)]
pub struct Mouse {
    pub x: i16,
    pub y: i16,
    pub buttons: u8,
}

impl Mouse {
    pub fn new() -> Self {
        let mut mouse = Mouse {
            x: 0,
            y: 0,
            buttons: 0,
        };
        mouse.update();
        mouse
    }
    fn update(&mut self) {
        unsafe {
            self.x = *MOUSE_X;
            self.y = *MOUSE_Y;
            self.buttons = *MOUSE_BUTTONS;
        }
    }
}

pub struct GamePad {
    game_pad_ptr: *const u8,
    pub button_1: bool,
    pub button_2: bool,
    pub up: bool,
    pub left: bool,
    pub down: bool,
    pub right: bool,
}

impl GamePad {
    pub fn new(number: u8) -> Self {
        let base_ptr = (0x16 as *const u8);
        let mut game_pad = GamePad {
            game_pad_ptr: unsafe { base_ptr.offset(number as isize) },
            button_1: false,
            button_2: false,
            up: false,
            left: false,
            down: false,
            right: false,
        };
        game_pad.update();
        game_pad
    }
    fn update(&mut self) {
        unsafe {
            self.button_1 = (*self.game_pad_ptr & BUTTON_1) != 0;
            self.button_2 = (*self.game_pad_ptr & BUTTON_2) != 0;
            self.left = (*self.game_pad_ptr & BUTTON_LEFT) != 0;
            self.right = (*self.game_pad_ptr & BUTTON_RIGHT) != 0;
            self.up = (*self.game_pad_ptr & BUTTON_UP) != 0;
            self.down = (*self.game_pad_ptr & BUTTON_DOWN) != 0;
        }
    }
}
