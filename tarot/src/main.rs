use bevy::{
    asset::{AssetLoader, LoadContext, LoadedAsset},
    prelude::*,
    reflect::TypeUuid,
    utils::BoxedFuture,
};
use bevy_asset_ron::*;
use serde::Deserialize;

use anyhow;
use ron;
mod game;

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
struct CatalogAsset {
    title: String,
    cards: Vec<CardAsset>,
}

#[derive(Debug, Deserialize, TypeUuid)]
#[uuid = "39cadc56-aa9c-4543-8640-a018b74b5052"]
struct CardAsset {
    order: u32,
    title: String,
    up: String,
    reverse: String,
}

#[derive(Default)]
pub struct CustomAssetLoader;

impl AssetLoader for CustomAssetLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), anyhow::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<CatalogAsset>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
#[derive(Default)]
struct State {
    handle: Handle<CatalogAsset>,
    printed: bool,
}

fn setup(mut state: ResMut<State>, asset_server: Res<AssetServer>) {
    state.handle = asset_server.load("cards/modern-magick.ron");
}

fn print_on_load(mut state: ResMut<State>, custom_assets: ResMut<Assets<CatalogAsset>>) {
    let custom_asset = custom_assets.get(&state.handle);
    if state.printed || custom_asset.is_none() {
        return;
    }

    info!("Custom asset loaded: {:?}", custom_asset.unwrap());
    state.printed = true;
}

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
        .init_resource::<State>()
        .insert_resource(game::Cards::default())
        .insert_resource(game::Shoe::default())
        .add_plugin(RonAssetPlugin::<CatalogAsset>::new(&["ron"]))
        .add_event::<game::CardFlipEvent>()
        .add_startup_system(game::tarot::setup)
        .add_system(game::tarot::flip_card)
        .add_system(game::tarot::handle_mouse_clicks)
        .add_startup_stage(
            "game_setup",
            SystemStage::single(game::tarot::spawn_card.system()),
        )
        .add_startup_system(setup)
        .add_system(print_on_load)
        .run();
}
