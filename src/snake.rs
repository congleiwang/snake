use bevy::prelude::*;
use crate::movement::Direction;
use crate::{Materials};
use crate::background::{Position, Size};
use crate::food::Food;

#[derive(Component)]
pub struct SnakeHead {
    pub direction: Direction,
}

#[derive(Component)]
pub struct SnakeSegment;

#[derive(Component, Default)]
pub struct SnakeSegments(pub Vec<Entity>);

#[derive(Component)]
pub struct GrowthEvent;

#[derive(Component, Default)]
pub struct LastTailPosition(pub Option<Position>);

pub fn spawn_snake(mut commands: Commands, materials: Res<Materials>, mut segments: ResMut<SnakeSegments>) {
    segments.0 = vec![
        commands.spawn_bundle(SpriteBundle {
            // 生成一个30*30px大小的2d方块
            sprite: Sprite {
                // tmp
                custom_size: Some(Vec2::new(30.0, 30.0)),
                color: Color::rgb(0.7, 0.7, 0.7),
                ..Default::default()
            },
            ..Default::default()
        })
            .insert(SnakeHead { direction: Direction::Up })
            .insert(SnakeSegment)
            .insert(Position::new(3, 3))
            .insert(Size::square(0.8))
            .id(),
        spawn_segment(
            commands,
            &materials.segment_material,
            Position::new(3, 2),
        ),
    ];
}

pub fn spawn_segment(mut commands: Commands, material: &Handle<ColorMaterial>, position: Position) -> Entity {
    commands.spawn_bundle(SpriteBundle {
        sprite: Sprite {
            color: Color::rgb(0.3, 0.3, 0.3),
            ..Default::default()
        },
        ..Default::default()
    }).insert(SnakeSegment)
        .insert(position)
        .insert(Size::square(0.65))
        .id()
}


pub fn snake_eating(mut commands: Commands, mut growth_writer: EventWriter<GrowthEvent>,
                    food_positions: Query<(Entity, &Position), With<Food>>,
                    head_positions: Query<&Position, With<SnakeHead>>) {
    for head_pos in head_positions.iter() {
        for (entity, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(entity).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

pub fn snake_growth(comands: Commands, last_tail_position: Res<LastTailPosition>,
                    mut segements: ResMut<SnakeSegments>, mut growth_reader: EventReader<GrowthEvent>,
                    materials: Res<Materials>) {
    if growth_reader.iter().next().is_some() {
        segements.0.push(spawn_segment(
            comands,
            &materials.segment_material,
            last_tail_position.0.unwrap(),
        ))
    }
}