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
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/cards.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(129., 129.), 6, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //let mut cards = vec![];
    //for i in 0..1 {
    //    let image_path: &str = &format!("images/{order}.png", order = i);
    //    let image = asset_server.load(image_path);
    //    cards.push(image);
    //}

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    //commands.insert_resource(super::Materials {
    //    cover: asset_server.load("images/cover.png"),
    //    cards: cards,
    //});
    commands
    .spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform::from_scale(Vec3::splat(2.0)),
        ..Default::default()
    })
    .insert(Timer::from_seconds(1., true));
}

pub fn spawn_card(
    mut query: Query<(&mut TextureAtlasSprite)>,
) {
    let mut sprite = query.single_mut();
    sprite.index = 23;
}

pub fn animate_sprite_system(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &Handle<TextureAtlas>)>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in query.iter_mut() {
        timer.tick(time.delta());
        if timer.finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
