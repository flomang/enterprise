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

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = asset_server.load("images/cards.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(super::CARD_WIDTH, super::CARD_HEIGHT), 6, 4);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    //let mut cards = vec![];
    //for i in 0..1 {
    //    let image_path: &str = &format!("images/{order}.png", order = i);
    //    let image = asset_server.load(image_path);
    //    cards.push(image);
    //}

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    //commands.insert_resource(super::Materials {
    //    cover: asset_server.load("images/cover.png"),
    //    cards: cards,
    //});
    commands.spawn_bundle(SpriteSheetBundle {
        texture_atlas: texture_atlas_handle,
        transform: Transform {
            translation: Vec3::new(super::CARD_WIDTH, super::CARD_HEIGHT, 0.),
            rotation: Quat::IDENTITY,
            scale: Vec3::splat(2.0),
        },
        ..Default::default()
    });

    commands.insert_resource(super::Card {
        flip_card: false,
        flipped: false,
    });
}

pub fn spawn_card(mut query: Query<(&mut TextureAtlasSprite)>) {
    let mut sprite = query.single_mut();
    sprite.index = 23;
}

pub fn flip_card(
    mut card: ResMut<super::Card>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform)>,
) {
    let (mut sprite, mut transform) = query.single_mut();

    if card.flip_card {
        if !card.flipped {
            transform.scale.x -= 1.;

            if transform.scale.x < 0.0 {
                transform.scale.x = 0.0;
                card.flipped = true;
                sprite.index = 1;
            }
        } else {
            transform.scale.x += 1.;

            if transform.scale.x >= 2.0 {
                transform.scale.x = 2.0;
                //card.flipped = alse;
                //card.flip_card = false;
                //sprite.index = 23;
            }
        }
    }

    if !card.flip_card {
        if card.flipped {
            transform.scale.x -= 1.;

            if transform.scale.x < 0.0 {
                transform.scale.x = 0.0;
                card.flipped = false;
                sprite.index = 23;
            }
        } else {
            transform.scale.x += 1.;

            if transform.scale.x >= 2.0 {
                transform.scale.x = 2.0;
                //card.flipped = alse;
                //card.flip_card = false;
                //sprite.index = 23;
            }
        }
    }
}

pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut card: ResMut<super::Card>,
    mut query: Query<(&TextureAtlasSprite, &GlobalTransform)>,
    q_camera: Query<&Transform, With<MainCamera>>,
) {
    let win = windows.get_primary().expect("no primary window");
    //let (_, transform) = query.single_mut();

    if mouse_input.just_pressed(MouseButton::Left) {
        //card.flip_card = !card.flip_card;
        //println!("click at {:?}", win.cursor_position());

        if let Some(pos) = win.cursor_position() {
            let (_, transform) = query.single_mut();
            let x1 = transform.translation.x - 129.0;
            let y1 = transform.translation.y - 129.0;
            let x2 = transform.translation.x + 129.0;
            let y2 = transform.translation.y + 129.0;
            println!("sprite ({}, {})", transform.translation.x - 129., transform.translation.y - 129.);

            // get the size of the window
            let size = Vec2::new(win.width() as f32, win.height() as f32);
            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;
            // assuming there is exactly one main camera entity, so this is OK
            let camera_transform = q_camera.single();
            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            eprintln!("World coords: x={} y={}", pos_wld.x, pos_wld.y);

            if pos_wld.x > x1 && pos_wld.x < x2 && pos_wld.y > y1 && pos_wld.y < y2 {
                card.flip_card = !card.flip_card;
            }
        }
    }
}
