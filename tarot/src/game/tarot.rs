use bevy::prelude::*;
use rand::Rng;

/// Used to help identify our main camera
#[derive(Component)]
pub struct MainCamera;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut shoe: ResMut<super::Shoe>,
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
        card_catalog: asset_server.load("cards/modern-magick.ron"),
    });

    // init shoe values
    let vec: Vec<usize> = (0..22).map(|x| x as usize).collect();
    shoe.0 = vec;
}

pub fn spawn_card(
    mut commands: Commands,
    materials: Res<super::Materials>,
    mut cards: ResMut<super::Cards>,
) {
    for i in 0..3 {
        let card = super::Card {
            state: super::CardState::Down,
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
}

pub fn flip_card(
    //mut reader: EventReader<super::CardFlipEvent>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut super::Card)>,
    mut shoe: ResMut<super::Shoe>,
    materials: Res<super::Materials>,
    catalog_assets: ResMut<Assets<super::CatalogAsset>>,
) {
    //if reader.iter().next().is_some() {
    for (mut sprite, mut transform, mut card) in query.iter_mut() {
        match card.state {
            super::CardState::FlipUp => {
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
                    card.state = super::CardState::TransitionUp;
                    // plus 1 here to skip the first sprite in the sheet
                    sprite.index = card_index + 1;
                    transform.rotation = Quat::from_rotation_z(radians);

                    let custom_asset = catalog_assets.get(&materials.card_catalog);
                    let card_asset = custom_asset.unwrap();
                    let el_asset = &card_asset.cards[card_index];
                    let order = &el_asset.order;
                    let title = &el_asset.title;
                    if radians > 0.0 {
                        info!("{} ({}) Reverse: {}", title, order, el_asset.reverse);
                    } else {
                        info!("{} ({}) Up: {}", title, order, el_asset.up);
                    };
                }
            }
            super::CardState::TransitionUp => {
                transform.scale.x += 1.;

                if transform.scale.x >= 2.0 {
                    transform.scale.x = 2.0;
                    card.state = super::CardState::Up;
                }
            }
            super::CardState::FlipDown => {
                transform.scale.x -= 1.;

                if transform.scale.x < 0.0 {
                    transform.scale.x = 0.0;
                    // return card index to shoe
                    // remember the index is offset by 1 in the spritesheet
                    shoe.0.push(sprite.index - 1);
                    // show card cover
                    sprite.index = 23;
                    card.state = super::CardState::TransitionDown;
                }
            }
            super::CardState::TransitionDown => {
                transform.scale.x += 1.;

                if transform.scale.x >= 2.0 {
                    transform.scale.x = 2.0;
                    card.state = super::CardState::Down;
                }
            }
            _ => (),
        }
    }
}

pub fn handle_mouse_clicks(
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut query: Query<(Entity, &mut super::Card)>,
    q_camera: Query<&Transform, With<MainCamera>>,
    mut card_flip_writer: EventWriter<super::CardFlipEvent>,
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

            for (entity, mut card) in query.iter_mut() {
                if card.rect.contains(pos_wld.x, pos_wld.y) {
                    if card.state == super::CardState::Down {
                        card.state = super::CardState::FlipUp;
                    } else if card.state == super::CardState::Up {
                        card.state = super::CardState::FlipDown;
                    }
                    card_flip_writer.send(super::CardFlipEvent { entity: entity });
                }
            }
        }
    }
}
