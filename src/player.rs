use crate::controls::GamePad;

pub struct Player {
    x: f64,
    y: f64,
}

static mut player: Player = Player { x: 0.0, y: 0.0 };

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
        }
    }
    pub fn get_x() -> f64 {
        unsafe { player.x }
    }

    pub fn get_y() -> f64 {
        unsafe { player.y }
    }
}
