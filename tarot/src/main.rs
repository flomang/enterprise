use bevy::prelude::*;
use bevy_easings::*;
use bevy_asset_ron::*;

mod game;
use game::menu;
use game::splash;
use game::tarot;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(RonAssetPlugin::<game::CatalogAsset>::new(&["ron"]))
        .add_plugin(bevy_easings::EasingsPlugin)
        .insert_resource(ClearColor(Color::rgb(0.09, 0.09, 0.09)))
        .insert_resource(game::DisplayQuality::Medium)
        .insert_resource(game::Volume(7))
        .insert_resource(game::Cards::default())
        .insert_resource(game::Shoe::default())
        .add_event::<game::CardFlipEvent>()
        .add_startup_system(setup)
        .add_state(game::GameState::Splash)
        .add_plugin(splash::SplashPlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(tarot::GamePlugin)
        .run();
}

// As there isn't an actual game, setup is just adding a `UiCameraBundle`
fn setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}
