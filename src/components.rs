use bevy::prelude::*;

#[derive(Component)]
pub struct CustomUV;

#[derive(Component)]
pub struct MousePos {
    pub prev_pos: Vec2,
}

impl Default for MousePos {
    fn default() -> Self {
        Self {
            prev_pos: Vec2::ZERO,
        }
    }
}

impl MousePos {
    pub fn calculate_delta(&mut self, current_pos: Vec2) -> Vec2 {
        let delta = self.prev_pos - current_pos;
        delta
    }
}
