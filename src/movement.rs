use bevy::prelude::*;
use crate::{LastTailPosition, Materials, SnakeHead, SnakeSegment, SnakeSegments, spawn_snake};
use crate::background::{CELL_X_COUNT, CELL_Y_COUNT, Position};
use crate::food::Food;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn opposite(&self) -> Self {
        match &self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

#[derive(Component)]
pub struct GameOverEvent;

pub fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir = if keyboard_input.pressed(KeyCode::Left) {
            Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            Direction::Right
        } else {
            head.direction.clone()
        };
        if dir != head.direction.opposite() {
            head.direction = dir
        }
    }
}


pub fn snake_movement(segments: ResMut<SnakeSegments>, mut heads: Query<(Entity, &SnakeHead)>,
                      mut last_tail_position: ResMut<LastTailPosition>, mut game_over_writer: EventWriter<GameOverEvent>,
                      mut positions: Query<&mut Position>) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        // 先取出蛇身列表中的所有position
        let segment_positions = segments.0.iter().map(|e| *positions.get_mut(*e).unwrap()).collect::<Vec<Position>>();
        // 蛇头
        let mut head_pos = positions.get_mut(head_entity).unwrap();
        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };
        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= CELL_X_COUNT
            || head_pos.y as u32 >= CELL_Y_COUNT {
            game_over_writer.send(GameOverEvent);
        }
        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }

        segment_positions.iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos
            });
        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

pub fn game_over(mut commands: Commands, mut reader: EventReader<GameOverEvent>,
                 materials: Res<Materials>, segments_res: ResMut<SnakeSegments>,
                 food: Query<Entity, With<Food>>, segments: Query<Entity, With<SnakeSegment>>) {
    if reader.iter().next().is_some() {
        for entity in food.iter().chain(segments.iter()) {
            commands.entity(entity).despawn();
        }
        spawn_snake(commands, materials, segments_res);
    }
}