use rand::prelude::random;
use bevy::prelude::*;
use crate::background::{CELL_X_COUNT, CELL_Y_COUNT, Position, Size};
use crate::{Materials, SnakeSegments};

#[derive(Component)]
pub struct Food;

pub fn food_spawner(mut commands: Commands, _materials: Res<Materials>, foods: Query<&Food>, positions: Query<&Position, With<SnakeSegments>>) {
    match foods.iter().next() {
        None => {
            let mut x = (random::<f32>() * CELL_X_COUNT as f32) as i32;
            let mut y = (random::<f32>() * CELL_Y_COUNT as f32) as i32;
            loop {
                let mut check_pos = true;
                for position in positions.iter() {
                    if position.x == x && position.y == y {
                        check_pos = false;
                        x = (random::<f32>() * CELL_X_COUNT as f32) as i32;
                        y = (random::<f32>() * CELL_Y_COUNT as f32) as i32;
                        break;
                    }
                }
                if check_pos {
                    break;
                }
            }
            commands.spawn_bundle(SpriteBundle {
                // texture: materials.food_material.clone(),
                sprite: Sprite {
                    color: Color::rgb(1.0, 0.0, 1.0).into(),
                    ..Default::default()
                },
                ..Default::default()
            })
                .insert(Food)
                .insert(Position { x, y })
                .insert(Size::square(0.6));
        }
        _ => {}
    }
}