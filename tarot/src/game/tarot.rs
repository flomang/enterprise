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
) {
    let mut cards = vec![];
    for i in 0..1 {
        let image_path: &str = &format!("images/{order}.png", order = i);
        let image = asset_server.load(image_path);
        cards.push(image);
    }

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(super::Materials {
        cover: asset_server.load("images/cover.png"),
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
            //texture: materials.cards[0].clone(),
            sprite: Sprite {
                // Flip the logo to the left
                flip_x: false,
                // And don't flip it upside-down ( the default )
                flip_y: false,
                custom_size: Some(Vec2::new(150.0, 200.0)),
                ..Default::default()
            },
            ..Default::default()
        });
        //.insert(super::Card);
}