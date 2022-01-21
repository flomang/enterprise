use bevy::prelude::*;
use rand::Rng;

use super::{
    despawn_screen, GameState, BORDER, HOVERED_BUTTON, HOVERED_PRESSED_BUTTON, MENU, NORMAL_BUTTON,
    PRESSED_BUTTON, TEXT_COLOR,
};

// This plugin will contain the game. In this case, it's just be a screen that will
// display the current settings for 5 seconds before returning to the menu
pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup_materials)
            .add_system_set(
                SystemSet::on_enter(GameState::Game)
                    .with_system(setup_ui)
                    .with_system(setup_cards),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Game)
                    .with_system(handle_card_flip)
                    .with_system(handle_mouse_input)
                    .with_system(handle_menu_action)
                    .with_system(handle_button_colors),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Game)
                    .with_system(despawn_screen::<OnGameScreen>)
                    .with_system(despawn_screen::<MainCamera>)
                    .with_system(despawn_screen::<super::Card>),
            );
    }
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;

// All actions that can be triggered from a button click
#[derive(Component)]
enum GameActions {
    BackToMainMenu,
}

/// Used to help identify our main camera
#[derive(Component)]
struct MainCamera;

// Tag component used to mark wich setting is currently selected
#[derive(Component)]
struct SelectedOption;

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

fn setup_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    clear_color: ResMut<ClearColor>,
) {
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(MainCamera);

    let button_style = Style {
        size: Size::new(Val::Px(100.0), Val::Px(33.0)),
        margin: Rect::all(Val::Px(6.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..Default::default()
    };
    let button_text_style = TextStyle {
        font: asset_server.load("fonts/FiraSans-Bold.ttf"),
        font_size: 30.0,
        color: TEXT_COLOR,
    };

    // main game screen with left menu
    // For UI, the origin is at the bottom left corner.
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(101.0), Val::Percent(100.0)),
                border: Rect::all(Val::Px(1.0)),
                ..Default::default()
            },
            color: UiColor(clear_color.0),
            ..Default::default()
        })
        .insert(OnGameScreen)
        .with_children(|parent| {
            // left side menu
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Px(100.0), Val::Percent(100.0)),
                        border: Rect::all(Val::Px(1.0)),
                        ..Default::default()
                    },
                    color: BORDER.into(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(NodeBundle {
                            style: Style {
                                size: Size::new(Val::Percent(99.0), Val::Percent(100.0)),
                                ..Default::default()
                            },
                            color: MENU.into(),
                            ..Default::default()
                        })
                        .with_children(|parent| {
                            // quit button to exit the game
                            parent
                                .spawn_bundle(ButtonBundle {
                                    style: button_style,
                                    color: NORMAL_BUTTON.into(),
                                    ..Default::default()
                                })
                                .insert(GameActions::BackToMainMenu)
                                .with_children(|parent| {
                                    parent.spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            "Quit",
                                            button_text_style,
                                            Default::default(),
                                        ),
                                        ..Default::default()
                                    });
                                });
                        });
                });
        });
}

fn setup_cards(mut commands: Commands, mut shoe: ResMut<super::Shoe>, materials: Res<Materials>) {
    // init shoe values
    let vec: Vec<usize> = (0..22).map(|x| x as usize).collect();
    shoe.0 = vec;

    // these cards use 2D space
    // the origin (X=0.0; Y=0.0) is at the center of the screen by default
    for i in 0..3 {
        let card = super::Card {
            state: super::CardState::Down,
            rect: super::Rect {
                x: -200.0 + (super::CARD_WIDTH * i as f32 * 2.0),
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
            .insert(card);
    }
}

fn handle_card_flip(
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

fn handle_mouse_input(
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

fn handle_menu_action(
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

// This system handles changing all buttons color based on mouse interaction
fn handle_button_colors(
    mut interaction_query: Query<
        (&Interaction, &mut UiColor, Option<&SelectedOption>),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, selected) in interaction_query.iter_mut() {
        *color = match (*interaction, selected) {
            (Interaction::Clicked, _) => PRESSED_BUTTON.into(),
            (Interaction::Hovered, Some(_)) => HOVERED_PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => HOVERED_BUTTON.into(),
            (Interaction::None, Some(_)) => PRESSED_BUTTON.into(),
            (Interaction::None, None) => NORMAL_BUTTON.into(),
        }
    }
}
