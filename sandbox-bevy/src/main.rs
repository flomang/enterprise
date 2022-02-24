use bevy::core::FixedTimestep;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod snake;
use snake::events;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "Snake!".to_string(),
            width: snake::WINDOW_WIDTH,
            height: snake::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(snake::SnakeSegments::default())
        .insert_resource(snake::LastTailPosition::default())
        .add_event::<events::GrowthEvent>()
        .add_event::<events::GameOverEvent>()
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .add_startup_stage(
            "game_setup",
            SystemStage::single(snake::game::spawn_snake.system()),
        )
        .add_startup_system(snake::game::setup.system())
        .add_plugins(DefaultPlugins)
        .add_plugin(ShapePlugin)
        .add_system(
            snake::game::snake_movement_input
                .system()
                .label(snake::SnakeMovement::Input)
                .before(snake::SnakeMovement::Movement),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(3.3))
                .with_system(snake::game::spawn_food.system())
                .label(snake::FoodState::Spawn),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.5))
                .with_system(
                    snake::game::food_movement
                        .system()
                        .label(snake::FoodState::Movement),
                )
                .after(snake::FoodState::Spawn),
        )
        .add_system_set(
            SystemSet::new()
                .with_run_criteria(FixedTimestep::step(0.050))
                .with_system(
                     snake::game::snake_movement
                         .system()
                         .label(snake::SnakeMovement::Movement),
                )
                .with_system(
                    snake::game::snake_eating
                        .system()
                        .label(snake::SnakeMovement::Eating)
                        .after(snake::SnakeMovement::Movement)
                        .before(snake::FoodState::Movement),
                )
                .with_system(
                    snake::game::snake_dying
                        .system()
                        .after(snake::SnakeMovement::Movement),
                )
                .with_system(
                    snake::game::snake_growth
                        .system()
                        .label(snake::SnakeMovement::Growth)
                        .after(snake::SnakeMovement::Eating),
                ),
        )
        .add_system(
            snake::game::game_over
                .system()
                .after(snake::SnakeMovement::Movement),
        )
        .add_system_set_to_stage(
            CoreStage::PostUpdate,
            SystemSet::new()
                .with_system(snake::game::position_translation.system())
        )
        .run();
}
