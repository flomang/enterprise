use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::random;


fn random_position(
    entities: Query<Entity, With<super::Position>>,
    mut positions: Query<&mut super::Position>,
) -> Option<super::Position> {
    let entity_positions = entities
        .iter()
        .map(|e| *positions.get_mut(e).unwrap())
        .collect::<Vec<super::Position>>();

    let position = super::Position {
        x: super::ARENA_WIDTH as i32,
        //x: (random::<f32>() * super::ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * super::ARENA_HEIGHT as f32) as i32,
    };

    match entity_positions.contains(&position) {
        true => None,
        false => Some(position),
    }
}

fn sprite_factory(material: &Handle<ColorMaterial>) -> SpriteBundle {
    let transform = Transform::from_translation(Vec3::new(-400., 0., 1.));
    SpriteBundle {
        material: material.clone(),
        sprite: Sprite::new(Vec2::new(300., 300.)),
        transform,
        ..Default::default()
    }
}


pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let fool_handle = asset_server.load("images/sigil.png");

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(super::Materials {
        card_cover: materials.add(fool_handle.into()),
    });
}


pub fn spawn_card(
    mut commands: Commands,
    materials: Res<super::Materials>,
    entities: Query<Entity, With<super::Position>>,
    positions: Query<&mut super::Position>,
) {
    // can't spawn on existing entity
    if let Some(position) = random_position(entities, positions) {
        commands
            .spawn_bundle(sprite_factory(&materials.card_cover))
            .insert(super::Card)
            .insert(super::Position{ x: 30, y: 30});
    }
}

pub fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / super::ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / super::ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

pub fn position_translation(
    windows: Res<Windows>,
    mut q: Query<(&super::Position, &mut Transform)>,
) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(
                pos.x as f32,
                window.width() as f32,
                super::ARENA_WIDTH as f32,
            ),
            convert(
                pos.y as f32,
                window.height() as f32,
                super::ARENA_HEIGHT as f32,
            ),
            0.0,
        );
    }
}