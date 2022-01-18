use bevy::prelude::*;
use bevy_asset_ron::*;

mod game;
use game::menu;
use game::splash;
use game::plugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RonAssetPlugin::<game::CatalogAsset>::new(&["ron"]))
        // Insert as resource the initial value for the settings resources
        .insert_resource(game::DisplayQuality::Medium)
        .insert_resource(game::Volume(7))
        .insert_resource(WindowDescriptor {
            title: "Tarot".to_string(),
            width: game::WINDOW_WIDTH,
            height: game::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(game::Cards::default())
        .insert_resource(game::Shoe::default())
        .add_event::<game::CardFlipEvent>()
        // add the app state type
        .add_startup_system(setup)
        // Declare the game state, and set its startup value
        .add_state(game::GameState::Splash)
        // Adds the plugins for each state
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(plugin::GamePlugin)
        .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
