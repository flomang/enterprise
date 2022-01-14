use bevy::prelude::*;
use rand::Rng;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let texture_handle = asset_server.load("images/cards.png");
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        Vec2::new(super::CARD_WIDTH, super::CARD_HEIGHT),
        6,
        4,
    );

    commands.insert_resource(super::Materials {
        sprite_sheet: texture_atlases.add(texture_atlas).into(),
    });
}

pub fn spawn_card(
    mut commands: Commands,
    materials: Res<super::Materials>,
    mut cards: ResMut<super::Cards>,
    mut shoe: ResMut<super::Shoe>,
) {
    for i in 0..3 {
        let card = super::Card {
            flip_card: false,
            flipped: false,
            rect: super::Rect {
                x: -250.0 + (super::CARD_WIDTH * i as f32 * 2.0),
                y: 0.0,
                width: super::CARD_WIDTH,
                height: super::CARD_HEIGHT,
            },
        };

        let entity = commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: TextureAtlasSprite {
                    index: 23,
                    ..Default::default()
                },
                texture_atlas: materials.sprite_sheet.clone(),
                transform: Transform {
                    translation: Vec3::new(card.rect.x, card.rect.y, i as f32),
                    scale: Vec3::new(2.0, 2.0, 1.0),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(card)
            .id();

        cards.0.push(entity);
    }
    let mut vec = Vec::new();
    for i in 0..22 {
        vec.push(i as usize);
    }
    shoe.0 = vec;

}

pub fn flip_card(
    //mut reader: EventReader<super::CardFlipEvent>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut super::Card)>,
    mut shoe: ResMut<super::Shoe>,
) {
    //if reader.iter().next().is_some() {
    for (mut sprite, mut transform, mut card) in query.iter_mut() {
        if card.flip_card {
            if !card.flipped {
                transform.scale.x -= 1.;

                if transform.scale.x < 0.0 {
                    // random up or reversed orientation
                    let radians = if rand::random() {
                        std::f32::consts::PI
                    } else {
                        0.0
                    };
                    let card_num = shoe.0.len();
                    let shoe_index = rand::thread_rng().gen_range(0..card_num); 
                    let card_index = shoe.0.remove(shoe_index);

                    transform.scale.x = 0.0;
                    card.flipped = true;
                    // plus 1 here to skip the first sprite in the sheet
                    sprite.index = card_index + 1;
                    transform.rotation = Quat::from_rotation_z(radians);
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
                    // return card index to shoe
                    // remember the index is offset by 1 in the spritesheet
                    shoe.0.push(sprite.index - 1);
                    // show card cover
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
}

pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<&mut super::Card>,
    q_camera: Query<&Transform, With<MainCamera>>,
    //mut card_flip_writer: EventWriter<super::CardFlipEvent>,
) {
    let win = windows.get_primary().expect("no primary window");

    if mouse_input.just_pressed(MouseButton::Left) {
        if let Some(pos) = win.cursor_position() {
            // get the size of the window
            let size = Vec2::new(win.width() as f32, win.height() as f32);
            // the default orthographic projection is in pixels from the center;
            // just undo the translation
            let p = pos - size / 2.0;
            // assuming there is exactly one main camera entity, so this is OK
            let camera_transform = q_camera.single();
            // apply the camera transform
            let pos_wld = camera_transform.compute_matrix() * p.extend(0.0).extend(1.0);
            //eprintln!("World coords: x={} y={}", pos_wld.x, pos_wld.y);

            for mut card in query.iter_mut() {
                //let mut card = query.single_mut();
                let x1 = card.rect.x - card.rect.width;
                let y1 = card.rect.y - card.rect.height;
                let x2 = card.rect.x + card.rect.width;
                let y2 = card.rect.y + card.rect.height;

                if pos_wld.x > x1 && pos_wld.x < x2 && pos_wld.y > y1 && pos_wld.y < y2 {
                    card.flip_card = !card.flip_card;
                    //card_flip_writer.send(super::CardFlipEvent);
                }
            }
        }
    }
}
