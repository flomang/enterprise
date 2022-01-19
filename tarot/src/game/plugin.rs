use bevy::prelude::*;
use rand::Rng;

use super::{despawn_screen, DisplayQuality, GameState, Volume, TEXT_COLOR};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_materials)
            .add_system_set(SystemSet::on_enter(GameState::Game).with_system(setup))
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(flip_card)
                    .with_system(handle_mouse_clicks)
                    .with_system(menu_action),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_screen::<OnGameScreen>)
                    .with_system(despawn_screen::<MainCamera>),
            );
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
// All actions that can be triggered from a button click
#[derive(Component)]
enum GameActions {
    BackToMainMenu,
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

struct Materials {
    sprite_sheet: Handle<TextureAtlas>,
    card_catalog: Handle<super::CatalogAsset>,
}

fn setup_materials(
    mut commands: Commands,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    asset_server: Res<AssetServer>,
) {
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
    commands.insert_resource(material);
}

fn setup(
    mut commands: Commands,
    mut shoe: ResMut<super::Shoe>,
    asset_server: Res<AssetServer>,
    materials: Res<Materials>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    // init shoe values
    let vec: Vec<usize> = (0..22).map(|x| x as usize).collect();
    shoe.0 = vec;

    let button_style = Style {
        size: Size::new(Val::Px(200.0), Val::Px(65.0)),
        margin: Rect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: TEXT_COLOR,
    };

    let gamescreen = commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                margin: Rect::all(Val::Auto),
                ..Default::default()
            },
            color: Color::BLACK.into(),
            ..Default::default()
        })
        .insert(OnGameScreen)
        .id();

    let menu = commands
        .spawn_bundle(ButtonBundle {
            style: button_style,
            color: NORMAL_BUTTON.into(),
            ..Default::default()
        })
        .insert(GameActions::BackToMainMenu)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text::with_section("Back", button_text_style, Default::default()),
                ..Default::default()
            });
        })
        .id();

    commands.entity(gamescreen).push_children(&[menu]);

    for i in 0..4 {
        let card = super::Card {
            state: super::CardState::Down,
            rect: super::Rect {
                x: -390.0 + (super::CARD_WIDTH * i as f32 * 2.0),
                y: 0.0,
                width: super::CARD_WIDTH,
                height: super::CARD_HEIGHT,
            },
        };

        commands
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
    mut query: Query<(Entity, &mut super::Card)>,
    mut card_flip_writer: EventWriter<super::CardFlipEvent>,
    mouse_input: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    q_camera: Query<&Transform, With<MainCamera>>,
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

fn menu_action(
    interaction_query: Query<(&Interaction, &GameActions), (Changed<Interaction>, With<Button>)>,
    mut game_state: ResMut<State<GameState>>,
) {
    for (interaction, menu_button_action) in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            match menu_button_action {
                GameActions::BackToMainMenu => game_state.set(GameState::Menu).unwrap(),
            }
        }
    }
}
