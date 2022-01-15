use bevy::prelude::*;
use bevy_asset_ron::*;
mod game;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(WindowDescriptor {
            title: "Tarot".to_string(),
            width: game::WINDOW_WIDTH,
            height: game::WINDOW_HEIGHT,
            ..Default::default()
        })
        .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
        .insert_resource(game::Cards::default())
        .insert_resource(game::Shoe::default())
        .add_plugin(RonAssetPlugin::<game::CatalogAsset>::new(&["ron"]))
        .add_event::<game::CardFlipEvent>()
        .add_startup_system(game::tarot::setup)
        .add_system(game::tarot::flip_card)
        .add_system(game::tarot::handle_mouse_clicks)
        .add_startup_stage(
            "game_setup",
            SystemStage::single(game::tarot::spawn_card.system()),
        )
        .run();
}
