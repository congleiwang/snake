use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

pub fn draw_center_cross(windows: Res<Windows>, mut lines: ResMut<DebugLines>) {
    let window = windows.get_primary().unwrap();
    let half_win_width = 0.5 * window.width();
    let half_win_height = 0.5 * window.height();
    // 横线
    lines.line(
        Vec3::new(-1.0 * half_win_width, 0.0, 0.0),
        Vec3::new(half_win_width, 0.0, 0.0),
        0.0,
    );
    // 竖线
    lines.line(
        Vec3::new(0.0, -1.0 * half_win_height, 0.0),
        Vec3::new(0.0, half_win_height, 0.0),
        0.0,
    );
}

/// 网格
pub fn draw_grid(windows: Res<Windows>, mut lines: ResMut<DebugLines>) {
    if let Some(window) = windows.get_primary() {
        let half_win_width = 0.5 * window.width();
        let half_win_height = 0.5 * window.height();
        let x_space = window.width() / CELL_X_COUNT as f32;
        let y_space = window.height() / CELL_Y_COUNT as f32;
        let mut i = -1.0 * half_win_height;
        while i < half_win_height {
            lines.line(
                Vec3::new(-1.0 * half_win_width, i, 0.0),
                Vec3::new(half_win_width, i, 0.0),
                0.0,
            );
            i += y_space;
        }
        let mut i = -1.0 * half_win_width;
        while i < half_win_width {
            lines.line(
                Vec3::new(i, -1.0 * half_win_height, 0.0),
                Vec3::new(i, half_win_height, 0.0),
                0.0,
            );
            i += x_space;
        }
        lines.line(
            Vec3::new(0.0, -1.0 * half_win_height, 0.0),
            Vec3::new(0.0, half_win_height, 0.0),
            0.0,
        );
    }
}


pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    if let Some(window) = windows.get_primary() {
        for (sprite_size, mut sprite) in q.iter_mut() {
            sprite.custom_size = Some(Vec2::new(
                sprite_size.width * (window.width() as f32 / CELL_X_COUNT as f32),
                sprite_size.height * (window.height() as f32 / CELL_Y_COUNT as f32),
            ));
        }
    }
}

pub fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn covert(pos: i32, window_size: f32, cell_count: u32) -> f32 {
        let tile_size = window_size / cell_count as f32;
        pos as f32 * tile_size - 0.5 * window_size + 0.5 * tile_size
    }
    if let Some(window) = windows.get_primary() {
        for (pos, mut transform) in q.iter_mut() {
            transform.translation = Vec3::new(
                covert(pos.x, window.width(), CELL_X_COUNT),
                covert(pos.y, window.height(), CELL_Y_COUNT),
                0.0,
            );
        }
    }
}

// 10 * 10 个网格
pub(crate) const CELL_X_COUNT: u32 = 30;
pub(crate) const CELL_Y_COUNT: u32 = 30;

/// 网格中的位置
#[derive(Default, Copy, Clone, Eq, PartialEq, Hash, Component)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Self {
        Position {
            x,
            y,
        }
    }
}

/// 蛇头在网格中的大小
#[derive(Component)]
pub struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Size {
            width: x,
            height: x,
        }
    }
}