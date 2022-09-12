mod food;
mod movement;
mod snake;
mod background;

use bevy::prelude::*;
use bevy::time::FixedTimestep;
use bevy_prototype_debug_lines::*;
use crate::background::{draw_center_cross, draw_grid, position_translation, size_scaling};
use crate::food::food_spawner;
use crate::movement::{game_over, GameOverEvent, snake_movement, snake_movement_input, SnakeMovement};
use crate::snake::{GrowthEvent, LastTailPosition, snake_eating, snake_growth, SnakeHead, SnakeSegment, SnakeSegments, spawn_snake};

pub struct Materials {
    head_material: Handle<ColorMaterial>,
    segment_material: Handle<ColorMaterial>,
    food_material: Handle<ColorMaterial>,
}

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "snake".to_string(),
            width: 800.0,
            height: 800.0,
            resizable: false,
            ..Default::default()
        })
        .insert_resource(ClearColor(
            Color::rgb(0.0, 0.0, 0.0)
        ))
        .insert_resource(SnakeSegments::default())
        .insert_resource(LastTailPosition::default())
        .add_startup_system(setup)
        .add_startup_stage("game_setup", SystemStage::single(spawn_snake))
        .add_system_to_stage(CoreStage::PostUpdate, position_translation,
        ).add_system_to_stage(CoreStage::PostUpdate, size_scaling)
        .add_system(draw_center_cross)
        .add_system(draw_grid)
        .add_system_set(SystemSet::new().label(SnakeMovement::Input).with_system(snake_movement_input).before(SnakeMovement::Movement))
        .add_system_set(SystemSet::new().label(SnakeMovement::Movement).with_system(snake_movement).with_run_criteria(FixedTimestep::step(0.3)))
        .add_system_set(SystemSet::new().label(SnakeMovement::Eating).with_system(snake_eating).after(SnakeMovement::Movement))
        .add_system_set(SystemSet::new().label(SnakeMovement::Growth).with_system(snake_growth).after(SnakeMovement::Eating))
        .add_system_set(SystemSet::new().with_system(game_over).after(SnakeMovement::Movement))
        .add_system_set(SystemSet::new().with_system(food_spawner).with_run_criteria(FixedTimestep::step(0.1)))
        .add_event::<GrowthEvent>()
        .add_event::<GameOverEvent>()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebugLinesPlugin::default())
        .run();
}


fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let mut camera = Camera2dBundle::default();
    camera.transform = Transform::from_translation(Vec3::new(0.0, 0.0, 5.0));
    commands.spawn_bundle(camera);
    commands.insert_resource(Materials {
        head_material: materials.add(Color::rgb(0.7, 0.7, 0.7).into()),
        segment_material: materials.add(Color::rgb(0.3, 0.3, 0.3).into()),
        food_material: materials.add(Color::rgb(1.0, 0.0, 1.0).into()),
    });
}

