use bevy::prelude::*;
//use bevy_prototype_lyon::prelude::*;
//use rand::prelude::random;

// fn random_position(
//     entities: Query<Entity, With<super::Position>>,
//     mut positions: Query<&mut super::Position>,
// ) -> Option<super::Position> {
//     let entity_positions = entities
//         .iter()
//         .map(|e| *positions.get_mut(e).unwrap())
//         .collect::<Vec<super::Position>>();

//     let position = super::Position {
//         //x: super::ARENA_WIDTH as i32,
//         x: (random::<f32>() * super::ARENA_WIDTH as f32) as i32,
//         y: (random::<f32>() * super::ARENA_HEIGHT as f32) as i32,
//     };

//     match entity_positions.contains(&position) {
//         true => None,
//         false => Some(position),
//     }
// }

// fn sprite_factory(material: &Handle<ColorMaterial>) -> SpriteBundle {
//     let transform = Transform {
//         translation: Vec3::new(-700., -400., 0.),
//         rotation: Quat::from_rotation_x(180.),
//         scale: Vec3::new(1., 1., 1.),
//     };

//     SpriteBundle {
//         material: material.clone(),
//         sprite: Sprite::new(Vec2::new(300., 400.)),
//         transform,
//         ..Default::default()
//     }
// }

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut cards = vec![];
    for i in 0..22 {
        let image_path: &str = &format!("images/{order}.png", order = i);
        let handle = asset_server.load(image_path);
        let card = materials.add(handle.into());
        cards.push(card);
    }
    let cover_handle = asset_server.load("images/cover.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(super::Materials {
        cover: cover_handle,
        cards: cards,
    });
}

pub fn spawn_card(
    mut commands: Commands,
    materials: Res<super::Materials>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            texture: materials.cover.clone(),
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: true,
                // And don't flip it upside-down ( the default )
                flip_y: false,
                ..Default::default()
            },
            ..Default::default()
        });
        //.insert(super::Card);
}

// pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
//     let window = windows.get_primary().unwrap();
//     for (sprite_size, mut sprite) in q.iter_mut() {
//         sprite.size = Vec2::new(
//             sprite_size.width / super::ARENA_WIDTH as f32 * window.width() as f32,
//             sprite_size.height / super::ARENA_HEIGHT as f32 * window.height() as f32,
//         );
//     }
// }

// pub fn position_translation(
//     windows: Res<Windows>,
//     mut q: Query<(&super::Position, &mut Transform)>,
// ) {
//     fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
//         let tile_size = bound_window / bound_game;
//         pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
//     }
//     let window = windows.get_primary().unwrap();
//     for (pos, mut transform) in q.iter_mut() {
//         transform.translation = Vec3::new(
//             convert(
//                 pos.x as f32,
//                 window.width() as f32,
//                 super::ARENA_WIDTH as f32,
//             ),
//             convert(
//                 pos.y as f32,
//                 window.height() as f32,
//                 super::ARENA_HEIGHT as f32,
//             ),
//             0.0,
//         );
//     }
// }
