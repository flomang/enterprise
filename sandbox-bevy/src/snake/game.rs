use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::random;

use super::events;
use super::Food;
use super::LastTailPosition;
use super::Poison;
use super::SnakeSegments;

fn regular_polygon_colored(sides: usize, radius: f32, outline: Color, fill: Color) -> super::Shape {
    super::Shape {
        shape: shapes::RegularPolygon {
            sides: sides,
            feature: shapes::RegularPolygonFeature::Radius(radius),
            ..shapes::RegularPolygon::default()
        },
        outline: outline,
        fill: fill,
    }
}

fn shape_factory(shape: &super::Shape) -> bevy_prototype_lyon::entity::ShapeBundle {
    GeometryBuilder::build_as(
        &shape.shape,
        ShapeColors::outlined(shape.fill, shape.outline),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(1.0),
        },
        Transform::default(),
    )
}

fn sprite_factory(material: &Handle<ColorMaterial>) -> SpriteBundle {
    let transform = Transform::from_translation(Vec3::new(-400., 0., 1.));
    SpriteBundle {
        material: material.clone(),
        sprite: Sprite::new(Vec2::new(24., 24.)),
        transform,
        ..Default::default()
    }
}

// return random optional position
// fn random_position(
//     entities: Query<Entity, With<super::Position>>,
//     mut positions: Query<&mut super::Position>,
// ) -> Option<super::Position> {
//     let entity_positions = entities
//         .iter()
//         .map(|e| *positions.get_mut(e).unwrap())
//         .collect::<Vec<super::Position>>();

//     let position = super::Position {
//         x: (random::<f32>() * super::ARENA_WIDTH as f32) as i32,
//         y: (random::<f32>() * super::ARENA_HEIGHT as f32) as i32,
//     };

//     match entity_positions.contains(&position) {
//         true => None,
//         false => Some(position),
//     }
// }

fn random_position_off_screen(
    entities: Query<Entity, With<super::Position>>,
    mut positions: Query<&mut super::Position>,
) -> Option<super::Position> {
    let entity_positions = entities
        .iter()
        .map(|e| *positions.get_mut(e).unwrap())
        .collect::<Vec<super::Position>>();

    let position = super::Position {
        x: super::ARENA_WIDTH as i32,
        y: (random::<f32>() * super::ARENA_HEIGHT as f32) as i32,
    };

    match entity_positions.contains(&position) {
        true => None,
        false => Some(position),
    }
}

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let pizza_handle = asset_server.load("images/neon-pizza-logo.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(super::Materials {
        snake_head: regular_polygon_colored(6, 5.0, Color::GREEN, Color::GREEN),
        snake_segment: regular_polygon_colored(6, 4.0, Color::GREEN, Color::GREEN),
        pizza: materials.add(pizza_handle.into()),
    });
}

pub fn spawn_snake_segment(
    mut commands: Commands,
    shape: &super::Shape,
    position: super::Position,
) -> Entity {
    commands
        .spawn_bundle(shape_factory(shape))
        .insert(super::SnakeSegment)
        .insert(position)
        .id()
}

pub fn spawn_snake(
    mut commands: Commands,
    materials: Res<super::Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
        commands
            .spawn_bundle(shape_factory(&materials.snake_head))
            .insert(super::SnakeHead {
                direction: super::Direction::Up,
                input_direction: super::Direction::Up,
            })
            .insert(super::SnakeSegment)
            .insert(super::Position { x: 3, y: 3 })
            .id(),
        spawn_snake_segment(
            commands,
            &materials.snake_segment,
            super::Position { x: 3, y: 2 },
        ),
    ];
}

pub fn spawn_food(
    mut commands: Commands,
    materials: Res<super::Materials>,
    entities: Query<Entity, With<super::Position>>,
    positions: Query<&mut super::Position>,
) {
    // can't spawn on existing entity
    if let Some(position) = random_position_off_screen(entities, positions) {
        commands
            .spawn_bundle(sprite_factory(&materials.pizza))
            .insert(Food)
            .insert(position);
    }
}

// pub fn spawn_poison(
//     mut commands: Commands,
//     materials: Res<super::Materials>,
//     entities: Query<Entity, With<super::Position>>,
//     positions: Query<&mut super::Position>,
// ) {
//     // can't spawn on existing entity
//     if let Some(position) = random_position(entities, positions) {
//         commands
//             .spawn_bundle(sprite_factory(&materials.pill))
//             .insert(super::Poison)
//             .insert(position);
//     }
// }

// pub fn spawn_wormhole(
//     mut commands: Commands,
//     materials: Res<super::Materials>,
//     entities: Query<Entity, With<super::Position>>,
//     positions: Query<&mut super::Position>,
// ) {
//     // can't spawn on existing entity
//     if let Some(position) = random_position(entities, positions) {
//         commands
//             .spawn_bundle(sprite_factory(&materials.cherry))
//             .insert(super::Wormhole)
//             .insert(position);
//     }
// }

pub fn food_movement(
    mut commands: Commands,
    mut query: Query<(Entity, &mut super::Position), With<Food>>,
){
    for (ent, mut pos) in query.iter_mut() {
        pos.x -= 1;

        // if offscreen despawssssss
        if pos.x < 0 {
            commands.entity(ent).despawn();
        }
    }
}

pub fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &super::SnakeHead)>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut super::Position>,
    mut game_over_writer: EventWriter<events::GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<super::Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            super::Direction::Left => {
                head_pos.x -= 1;
            }
            super::Direction::Right => {
                head_pos.x += 1;
            }
            super::Direction::Up => {
                head_pos.y += 1;
            }
            super::Direction::Down => {
                head_pos.y -= 1;
            }
        };

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= super::ARENA_WIDTH
            || head_pos.y as u32 >= super::ARENA_HEIGHT
        {
            game_over_writer.send(events::GameOverEvent);
        }

        if segment_positions.contains(&head_pos) {
            game_over_writer.send(events::GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });

        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

pub fn snake_movement_input(
    keyboard_input: Res<Input<KeyCode>>,
    mut heads: Query<&mut super::SnakeHead>,
) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: super::Direction = if keyboard_input.pressed(KeyCode::Left) {
            super::Direction::Left
        } else if keyboard_input.pressed(KeyCode::Down) {
            super::Direction::Down
        } else if keyboard_input.pressed(KeyCode::Up) {
            super::Direction::Up
        } else if keyboard_input.pressed(KeyCode::Right) {
            super::Direction::Right
        } else {
            head.direction
        };

        if dir != head.direction.opposite() && dir != head.input_direction.opposite() {
            head.input_direction = dir;
            head.direction = dir;
        }
    }
}

pub fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<events::GrowthEvent>,
    food_positions: Query<(Entity, &super::Position), With<Food>>,
    head_positions: Query<&super::Position, With<super::SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(events::GrowthEvent);
            }
        }
    }
}

// pub fn snake_transporting(
//     mut commands: Commands,
//     mut warp_writer: EventWriter<events::WarpEvent>,
//     warp_positions: Query<(Entity, &super::Position), With<super::Wormhole>>,
//     head_positions: Query<&super::Position, With<super::SnakeHead>>,
// ) {
//     for head_pos in head_positions.iter() {
//         for (ent, pos) in warp_positions.iter() {
//             if pos == head_pos {
//                 commands.entity(ent).despawn();
//                 warp_writer.send(events::WarpEvent);
//             }
//         }
//     }
// }

pub fn snake_dying(
    mut commands: Commands,
    mut game_over_writer: EventWriter<events::GameOverEvent>,
    poison_positions: Query<(Entity, &super::Position), With<Poison>>,
    head_positions: Query<&super::Position, With<super::SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, pos) in poison_positions.iter() {
            if pos == head_pos {
                commands.entity(ent).despawn();
                game_over_writer.send(events::GameOverEvent);
            }
        }
    }
}

pub fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<events::GrowthEvent>,
    materials: Res<super::Materials>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_snake_segment(
            commands,
            &materials.snake_segment,
            last_tail_position.0.unwrap(),
        ));
    }
}

// pub fn snake_warp(
//     mut commands: Commands,
//     mut segments: ResMut<SnakeSegments>,
//     mut warp_reader: EventReader<events::WarpEvent>,
// ) {
//     // on warp event and if there are more than 1 segments for the snake
//     if warp_reader.iter().next().is_some() && segments.0.len() > 1 {
//         if let Some(seg) = segments.0.pop() {
//             commands.entity(seg).despawn();
//         }
//     }
// }

pub fn game_over(
    mut commands: Commands,
    mut reader: EventReader<events::GameOverEvent>,
    materials: Res<super::Materials>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    segments: Query<Entity, With<super::SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        // TODO make this more readable
        for ent in food 
            .iter()
            .chain(segments.iter())
        {
            commands.entity(ent).despawn();
        }

        spawn_snake(commands, materials, segments_res);
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / super::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / super::ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

pub fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&super::Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width() as f32,
                super::ARENA_WIDTH as f32,
            ),
            convert(
                pos.y as f32,
                window.height() as f32,
                super::ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}
