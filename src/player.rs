use crate::controls::GamePad;

pub struct Player {
    x: f64,
    y: f64,
    z: f64,
}

static mut player: Player = Player {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

impl Player {
    pub fn update(game_pad: &GamePad) {
        unsafe {
            if (game_pad.down) {
                player.y += 0.25;
            }
            if (game_pad.up) {
                player.y -= 0.25;
            }
            if (game_pad.left) {
                player.x -= 0.25;
            }
            if (game_pad.right) {
                player.x += 0.25;
            }
            if (game_pad.button_1) {
                player.z += 0.25;
            }
            if (game_pad.button_2) {
                player.z -= 0.01;
            }
        }
    }
    pub fn get_x() -> f64 {
        unsafe { player.x }
    }

    pub fn get_y() -> f64 {
        unsafe { player.y }
    }

    pub fn get_z() -> f64 {
        unsafe { player.z }
    }
}
