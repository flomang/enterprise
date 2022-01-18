use bevy::prelude::*;
use rand::Rng;

use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(spawn_card))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(flip_card)
                    .with_system(handle_mouse_clicks),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game).with_system(despawn_screen::<OnGameScreen>),
            );
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

// struct GameTimer(Timer);

// fn game_setup(
//     mut commands: Commands,
//     asset_server: Res<AssetServer>,
//     display_quality: Res<DisplayQuality>,
//     volume: Res<Volume>,
// ) {
//     let font = asset_server.load("fonts/FiraSans-Bold.ttf");

//     commands
//         // First create a `NodeBundle` for centering what we want to display
//         .spawn_bundle(NodeBundle {
//             style: Style {
//                 // This will center the current node
//                 margin: Rect::all(Val::Auto),
//                 // This will display its children in a column, from top to bottom. Unlike
//                 // in Flexbox, Bevy origin is on bottom left, so the vertical axis is reversed
//                 flex_direction: FlexDirection::ColumnReverse,
//                 // `align_items` will align children on the cross axis. Here the main axis is
//                 // vertical (column), so the cross axis is horizontal. This will center the
//                 // children
//                 align_items: AlignItems::Center,
//                 ..Default::default()
//             },
//             color: Color::BLACK.into(),
//             ..Default::default()
//         })
//         .insert(OnGameScreen)
//         .with_children(|parent| {
//             // Display two lines of text, the second one with the current settings
//             parent.spawn_bundle(TextBundle {
//                 style: Style {
//                     margin: Rect::all(Val::Px(50.0)),
//                     ..Default::default()
//                 },
//                 text: Text::with_section(
//                     "Will be back to the menu shortly...",
//                     TextStyle {
//                         font: font.clone(),
//                         font_size: 80.0,
//                         color: TEXT_COLOR,
//                     },
//                     Default::default(),
//                 ),
//                 ..Default::default()
//             });
//             parent.spawn_bundle(TextBundle {
//                 style: Style {
//                     margin: Rect::all(Val::Px(50.0)),
//                     ..Default::default()
//                 },
//                 text: Text {
//                     sections: vec![
//                         TextSection {
//                             value: format!("quality: {:?}", *display_quality),
//                             style: TextStyle {
//                                 font: font.clone(),
//                                 font_size: 60.0,
//                                 color: Color::BLUE,
//                             },
//                         },
//                         TextSection {
//                             value: " - ".to_string(),
//                             style: TextStyle {
//                                 font: font.clone(),
//                                 font_size: 60.0,
//                                 color: TEXT_COLOR,
//                             },
//                         },
//                         TextSection {
//                             value: format!("volume: {:?}", *volume),
//                             style: TextStyle {
//                                 font: font.clone(),
//                                 font_size: 60.0,
//                                 color: Color::GREEN,
//                             },
//                         },
//                     ],
//                     ..Default::default()
//                 },
//                 ..Default::default()
//             });
//         });
//     // Spawn a 5 seconds timer to trigger going back to the menu
//     commands.insert_resource(GameTimer(Timer::from_seconds(5.0, false)));
// }

// // Tick the timer, and change state when finished
// fn game(time: Res<Time>, mut game_state: ResMut<State<GameState>>, mut timer: ResMut<GameTimer>) {
//     if timer.0.tick(time.delta()).finished() {
//         game_state.set(GameState::Menu).unwrap();
//     }
// }

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

struct Materials {
    sprite_sheet: Handle<TextureAtlas>,
    card_catalog: Handle<super::CatalogAsset>,
}

fn setup(
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

    let material = Materials {
        sprite_sheet: texture_atlases.add(texture_atlas).into(),
        card_catalog: asset_server.load("cards/modern-magick.ron"),
    };
    //materials.add(material);
    commands.insert_resource(material);

    // init shoe values
    let vec: Vec<usize> = (0..22).map(|x| x as usize).collect();
    shoe.0 = vec;
}

fn spawn_card(
    mut commands: Commands,
    materials: Res<Materials>,
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

fn exit(
    mut commands: Commands,
    //mut shoe: ResMut<super::Shoe>,
    cards: ResMut<super::Cards>,
) {
    for i in cards.0.iter() {
        commands.entity(*i).despawn();
    }
}

fn flip_card(
    //mut reader: EventReader<super::CardFlipEvent>,
    mut query: Query<(&mut TextureAtlasSprite, &mut Transform, &mut super::Card)>,
    mut shoe: ResMut<super::Shoe>,
    materials: Res<Materials>,
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

fn handle_mouse_clicks(
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
