use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy::render::pass::ClearColor;
use bevy::core::FixedTimestep;
use rand::prelude::random;

const WINDOW_WIDTH: f32 = 1000.0;
const WINDOW_HEIGHT: f32 = 1000.0;
const ARENA_WIDTH: u32 = 100;
const ARENA_HEIGHT: u32 = 100;

struct Food;
struct Poison;
struct Wormhole;
struct SnakeSegment;

struct GrowthEvent;
struct GameOverEvent;
struct WarpEvent;

#[derive(Default)]
struct SnakeSegments(Vec<Entity>);

#[derive(Default)]
struct Wormholes(Vec<Entity>);

#[derive(Default, Copy, Clone, Eq, PartialEq, Hash)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Default)]
struct LastTailPosition(Option<Position>);

#[derive(SystemLabel, Debug, Hash, PartialEq, Eq, Clone)]
pub enum SnakeMovement {
    Input,
    Movement,
    Eating,
    Growth,
}

struct SnakeHead {
    direction: Direction,
    input_direction: Direction,
}

struct Materials {
    head_shape: shapes::RegularPolygon,
    segment_shape: shapes::RegularPolygon,
    food_shape: shapes::RegularPolygon,
    poison_shape: shapes::RegularPolygon,
    wormhole_shape: shapes::RegularPolygon,
}


#[derive(PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Up,
    Right,
    Down,
}

impl Direction {
    fn opposite(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            Self::Up => Self::Down,
            Self::Down => Self::Up,
        }
    }
}

struct Size {
    width: f32,
    height: f32,
}

impl Size {
    pub fn square(x: f32) -> Self {
        Self {
            width: x,
            height: x,
        }
    }
}

fn setup(mut commands: Commands) {
    let snake_head = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(5.0),
        ..shapes::RegularPolygon::default()
    };
    let snake_segment = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(4.0),
        ..shapes::RegularPolygon::default()
    };
    let food = shapes::RegularPolygon {
        sides: 3,
        feature: shapes::RegularPolygonFeature::Radius(6.0),
        ..shapes::RegularPolygon::default()
    };
    let poison = shapes::RegularPolygon {
        sides: 8,
        feature: shapes::RegularPolygonFeature::Radius(6.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.insert_resource(Materials {
        head_shape: snake_head,
        segment_shape: snake_segment,
        food_shape: food,
        poison_shape: poison,
        wormhole_shape: shapes::RegularPolygon {
            sides: 12,
            feature: shapes::RegularPolygonFeature::Radius(6.0),
            ..shapes::RegularPolygon::default()
        },
    });
}

fn spawn_segment(
    mut commands: Commands,
    shape: shapes::RegularPolygon,
    position: Position,
) -> Entity {

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(Color::rgb(0.0, 1.0, 0.0), Color::BLACK),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(0.0),
            },
            Transform::default(),
        ))
        .insert(SnakeSegment)
        .insert(position)
        .id()
}

fn spawn_snake(
    mut commands: Commands,
    materials: Res<Materials>,
    mut segments: ResMut<SnakeSegments>,
) {
    segments.0 = vec![
        commands
            .spawn_bundle(GeometryBuilder::build_as(
                &materials.head_shape,
                ShapeColors::outlined(Color::GREEN, Color::BLACK),
                DrawMode::Outlined {
                    fill_options: FillOptions::default(),
                    outline_options: StrokeOptions::default().with_line_width(0.0),
                },
                Transform::default(),
            ))
            .insert(SnakeHead {
                direction: Direction::Up,
                input_direction: Direction::Up,
            })
            .insert(SnakeSegment)
            .insert(Position { x: 3, y: 3 })
            .id(),
        spawn_segment(
            commands,
            materials.segment_shape,
            Position { x: 3, y: 2 },
        ),
    ];
}

fn food_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    segments: ResMut<SnakeSegments>,
) {
    let segment_positions = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    let mut food_position = Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    };

    // food position can't be on the snake
    while segment_positions.contains(&food_position) {
        food_position = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };
    }

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &materials.food_shape,
            ShapeColors::outlined(Color::BLACK, Color::PURPLE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::default(),
        ))
        .insert(Food)
        .insert(food_position)
        .insert(Size::square(0.8));
}

fn poison_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    segments: ResMut<SnakeSegments>,
) {
    let segment_positions = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    let mut position = Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    };

    // food position can't be on the snake
    while segment_positions.contains(&position) {
        position = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };
    }

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &materials.poison_shape,
            ShapeColors::outlined(Color::BLACK, Color::RED),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::default(),
        ))
        .insert(Poison)
        .insert(position)
        .insert(Size::square(0.8));
}

fn wormhole_spawner(
    mut commands: Commands,
    materials: Res<Materials>,
    mut positions: Query<&mut Position>,
    segments: ResMut<SnakeSegments>,
) {
    let segment_positions = segments
        .0
        .iter()
        .map(|e| *positions.get_mut(*e).unwrap())
        .collect::<Vec<Position>>();

    let mut position = Position {
        x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
        y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
    };

    // can't spawn on snake 
    while segment_positions.contains(&position) {
        position = Position {
            x: (random::<f32>() * ARENA_WIDTH as f32) as i32,
            y: (random::<f32>() * ARENA_HEIGHT as f32) as i32,
        };
    }

    commands
        .spawn_bundle(GeometryBuilder::build_as(
            &materials.wormhole_shape,
            ShapeColors::outlined(Color::BLACK, Color::BLUE),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(1.0),
            },
            Transform::default(),
        ))
        .insert(Wormhole)
        .insert(position)
        .insert(Size::square(0.8));
}

fn snake_movement(
    segments: ResMut<SnakeSegments>,
    mut heads: Query<(Entity, &SnakeHead)>,
    mut last_tail_position: ResMut<LastTailPosition>,
    mut positions: Query<&mut Position>,
    mut game_over_writer: EventWriter<GameOverEvent>,
) {
    if let Some((head_entity, head)) = heads.iter_mut().next() {
        let segment_positions = segments
            .0
            .iter()
            .map(|e| *positions.get_mut(*e).unwrap())
            .collect::<Vec<Position>>();
        let mut head_pos = positions.get_mut(head_entity).unwrap();

        match &head.direction {
            Direction::Left => {
                head_pos.x -= 1;
            }
            Direction::Right => {
                head_pos.x += 1;
            }
            Direction::Up => {
                head_pos.y += 1;
            }
            Direction::Down => {
                head_pos.y -= 1;
            }
        };

        if head_pos.x < 0
            || head_pos.y < 0
            || head_pos.x as u32 >= ARENA_WIDTH
            || head_pos.y as u32 >= ARENA_HEIGHT
        {
            game_over_writer.send(GameOverEvent);
        }

        if segment_positions.contains(&head_pos) {
            game_over_writer.send(GameOverEvent);
        }

        segment_positions
            .iter()
            .zip(segments.0.iter().skip(1))
            .for_each(|(pos, segment)| {
                *positions.get_mut(*segment).unwrap() = *pos;
            });

        last_tail_position.0 = Some(*segment_positions.last().unwrap());
    }
}

fn snake_movement_input(keyboard_input: Res<Input<KeyCode>>, mut heads: Query<&mut SnakeHead>) {
    if let Some(mut head) = heads.iter_mut().next() {
        let dir: Direction = if keyboard_input.pressed(KeyCode::Left) {
                Direction::Left
            } else if keyboard_input.pressed(KeyCode::Down) {
                Direction::Down
            } else if keyboard_input.pressed(KeyCode::Up) {
                Direction::Up
            } else if keyboard_input.pressed(KeyCode::Right) {
                Direction::Right
            } else {
                head.direction
            };

        if dir != head.direction.opposite() && dir != head.input_direction.opposite() {
            head.input_direction = dir;
            head.direction = dir;
        }
    }
}

fn snake_eating(
    mut commands: Commands,
    mut growth_writer: EventWriter<GrowthEvent>,
    food_positions: Query<(Entity, &Position), With<Food>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, food_pos) in food_positions.iter() {
            if food_pos == head_pos {
                commands.entity(ent).despawn();
                growth_writer.send(GrowthEvent);
            }
        }
    }
}

fn snake_transporting(
    mut commands: Commands,
    mut warp_writer: EventWriter<WarpEvent>,
    warp_positions: Query<(Entity, &Position), With<Wormhole>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, pos) in warp_positions.iter() {
            if pos == head_pos {
                commands.entity(ent).despawn();
                warp_writer.send(WarpEvent);
            }
        }
    }
}

fn snake_dying(
    mut commands: Commands,
    mut game_over_writer: EventWriter<GameOverEvent>,
    poison_positions: Query<(Entity, &Position), With<Poison>>,
    head_positions: Query<&Position, With<SnakeHead>>,
) {
    for head_pos in head_positions.iter() {
        for (ent, pos) in poison_positions.iter() {
            if pos == head_pos {
                commands.entity(ent).despawn();
                game_over_writer.send(GameOverEvent);
            }
        }
    }
}


fn snake_growth(
    commands: Commands,
    last_tail_position: Res<LastTailPosition>,
    mut segments: ResMut<SnakeSegments>,
    mut growth_reader: EventReader<GrowthEvent>,
    materials: Res<Materials>,
) {
    if growth_reader.iter().next().is_some() {
        segments.0.push(spawn_segment(
            commands,
            materials.segment_shape,
            last_tail_position.0.unwrap(),
        ));
    }
}

fn snake_warp(
    _commands: Commands,
    mut warp_reader: EventReader<WarpEvent>,
) {
    if warp_reader.iter().next().is_some() {
        print!("do something!");
        //if let Some(seg) = segments.0.last() {
        //    //commands.entity(seg).despawn();
        //}
        //segments.0.push(spawn_segment(
        //    commands,
        //    materials.segment_shape,
        //    last_tail_position.0.unwrap(),
        //));
    }
}

fn game_over(
    mut commands: Commands,
    mut reader: EventReader<GameOverEvent>,
    materials: Res<Materials>,
    segments_res: ResMut<SnakeSegments>,
    food: Query<Entity, With<Food>>,
    poison: Query<Entity, With<Poison>>,
    wormhole: Query<Entity, With<Wormhole>>,
    segments: Query<Entity, With<SnakeSegment>>,
) {
    if reader.iter().next().is_some() {
        // TODO make this more readable
        for ent in wormhole.iter().chain(poison.iter().chain(food.iter().chain(segments.iter()))) {
            commands.entity(ent).despawn();
        }

        spawn_snake(commands, materials, segments_res);
    }
}

fn size_scaling(windows: Res<Windows>, mut q: Query<(&Size, &mut Sprite)>) {
    let window = windows.get_primary().unwrap();
    for (sprite_size, mut sprite) in q.iter_mut() {
        sprite.size = Vec2::new(
            sprite_size.width / ARENA_WIDTH as f32 * window.width() as f32,
            sprite_size.height / ARENA_HEIGHT as f32 * window.height() as f32,
        );
    }
}

fn position_translation(windows: Res<Windows>, mut q: Query<(&Position, &mut Transform)>) {
    fn convert(pos: f32, bound_window: f32, bound_game: f32) -> f32 {
        let tile_size = bound_window / bound_game;
        pos / bound_game * bound_window - (bound_window / 2.) + (tile_size / 2.)
    }
    let window = windows.get_primary().unwrap();
    for (pos, mut transform) in q.iter_mut() {
        transform.translation = Vec3::new(
            convert(pos.x as f32, window.width() as f32, ARENA_WIDTH as f32),
            convert(pos.y as f32, window.height() as f32, ARENA_HEIGHT as f32),
            0.0,
        );
    }
}

fn main() {
    App::build()
    .insert_resource( WindowDescriptor { 
        title: "Snake!".to_string(), 
        width: WINDOW_WIDTH,                 
        height: WINDOW_HEIGHT,                
        ..Default::default()         
    })
    .insert_resource(SnakeSegments::default()) 
    .insert_resource(LastTailPosition::default())
    .add_event::<GrowthEvent>()
    .add_event::<GameOverEvent>()
    .add_event::<WarpEvent>()
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(1.0))
            .with_system(food_spawner.system()),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(3.0))
            .with_system(poison_spawner.system()),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(3.0))
            .with_system(wormhole_spawner.system()),
    )
    .insert_resource(ClearColor(Color::rgb(0.04, 0.04, 0.04)))
    .add_startup_stage("game_setup", SystemStage::single(spawn_snake.system()))
    .add_startup_system(setup.system())
    .add_plugins(DefaultPlugins)
    .add_plugin(ShapePlugin)
    .add_system(
        snake_movement_input
            .system()
            .label(SnakeMovement::Input)
            .before(SnakeMovement::Movement),
    )
    .add_system_set(
        SystemSet::new()
            .with_run_criteria(FixedTimestep::step(0.050))
            .with_system(snake_movement.system().label(SnakeMovement::Movement))
            .with_system(
                snake_eating
                    .system()
                    .label(SnakeMovement::Eating)
                    .after(SnakeMovement::Movement),
            )
            .with_system(
                snake_dying
                    .system()
                    .after(SnakeMovement::Movement),
            )
            .with_system(
                snake_transporting
                    .system()
                    .after(SnakeMovement::Movement),
            )
            .with_system(
                snake_warp
                    .system()
                    .after(SnakeMovement::Movement),
            )
            .with_system(
                snake_growth
                    .system()
                    .label(SnakeMovement::Growth)
                    .after(SnakeMovement::Eating),
            )
    )
    .add_system(game_over.system().after(SnakeMovement::Movement))
    .add_system_set_to_stage(
        CoreStage::PostUpdate,
        SystemSet::new()
            .with_system(position_translation.system())
            .with_system(size_scaling.system()),
    )
    .run();
}