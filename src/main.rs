use bevy::{
    core_pipeline::clear_color::ClearColorConfig, prelude::*, sprite::MaterialMesh2dBundle,
};

// Window constants
const WINDOW_WIDTH: f32 = 640.0;
const WINDOW_HEIGHT: f32 = 480.0;
const WINDOW_TITLE: &str = "Pong";

// Game field constants
const MIDDLE_LINE_SEPARATOR_WIDTH: f32 = 10.0;
const MIDDLE_LINE_SEPARATOR_HEIGHT: f32 = 10.0;
const MIDDLE_LINE_SEPARATOR_GAP_HEIGHT: f32 = 10.0;

// Player constants
const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 80.0;
const PADDLE_STARTING_POSITION_X: f32 = 20.0;
const PADDLE_STARTING_POSITION_Y: f32 = 0.0;
const PADDLE_SPEED: f32 = 5.0;

// Ball constants
const BALL_STARTING_POSITION_X: f32 = 0.0;
const BALL_STARTING_POSITION_Y: f32 = 0.0;
const BALL_RADIUS: f32 = 10.0;
const BALL_VELOCITY: f32 = 150.0;

//TODO: Game Score constants
// score_font_size
// score_font_style
// score_player1_position_x
// score_player1_position_y
// score_player2_position_x
// score_player2_position_y

fn main() {
    App::new()
        // Setup default plugins with custom window settings
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: WINDOW_TITLE.to_string(),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        resizable: false,
                        resize_constraints: WindowResizeConstraints {
                            min_width: WINDOW_WIDTH,
                            min_height: WINDOW_HEIGHT,
                            max_width: WINDOW_WIDTH,
                            max_height: WINDOW_HEIGHT,
                        },
                        ..default()
                    }),
                    ..default()
                })
                .build(),
        )
        //.insert_resource(PlayerTimer(Timer::from_seconds(2.0, TimerMode::Repeating)))
        .add_systems(Startup, setup_camera)
        .add_systems(Startup, setup_middle_line)
        .add_systems(Startup, spawn_players)
        .add_systems(Startup, spawn_ball)
        .add_systems(Update, move_ball)
        .add_systems(Update, move_player)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        camera_2d: Camera2d {
            // Set window clear color to black
            clear_color: ClearColorConfig::Custom(Color::BLACK),
        },
        ..default()
    });
}

#[derive(Component, Clone)]
struct Player(String);

#[derive(Resource)]
struct PlayerTimer(Timer);

// Spawn the player paddles
fn spawn_players(
    mut command: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    command.spawn((
        Player("Player1".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                -WINDOW_WIDTH / 2.0 + PADDLE_WIDTH / 2.0 + PADDLE_STARTING_POSITION_X,
                PADDLE_STARTING_POSITION_Y,
                0.,
            )),
            ..default()
        },
    ));
    command.spawn((
        Player("Player2".to_string()),
        MaterialMesh2dBundle {
            mesh: meshes
                .add(shape::Box::new(PADDLE_WIDTH, PADDLE_HEIGHT, 0.).into())
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                WINDOW_WIDTH / 2.0 - PADDLE_WIDTH / 2.0 - PADDLE_STARTING_POSITION_X,
                PADDLE_STARTING_POSITION_Y,
                0.,
            )),
            ..default()
        },
    ));
}

fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut player: Query<(&mut Player, &mut Transform)>,
    //time_step: Res<FixedTime>,
) {
    for (player_name, mut transform) in &mut player {
        if &player_name.0[..] == "Player1" {
            //println!("Player1");
            if keyboard_input.pressed(KeyCode::W)
                && transform.translation.y < WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0
            {
                transform.translation.y += PADDLE_SPEED;
            }
            if keyboard_input.pressed(KeyCode::S)
                && transform.translation.y > -WINDOW_HEIGHT / 2.0 + PADDLE_HEIGHT / 2.0
            {
                transform.translation.y -= PADDLE_SPEED;
            }
        } else if &player_name.0[..] == "Player2" {
            //println!("Player2");
            if keyboard_input.pressed(KeyCode::Up)
                && transform.translation.y < WINDOW_HEIGHT / 2.0 - PADDLE_HEIGHT / 2.0
            {
                transform.translation.y += PADDLE_SPEED;
            }
            if keyboard_input.pressed(KeyCode::Down)
                && transform.translation.y > -WINDOW_HEIGHT / 2.0 + PADDLE_HEIGHT / 2.0
            {
                transform.translation.y -= PADDLE_SPEED;
            }
        }
    }
}

#[derive(Component)]
struct Ball;

#[derive(Component)]
enum BallDirection {
    Left,
    Right,
}

fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Ball
    commands.spawn((
        Ball,
        BallDirection::Left,
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                BALL_STARTING_POSITION_X,
                BALL_STARTING_POSITION_Y,
                0.,
            )),
            ..default()
        },
    ));
}

// TODO: Improve Ball movement system
fn move_ball(time: Res<Time>, mut ball_position: Query<(&mut BallDirection, &mut Transform)>) {
    for (mut direction, mut transform) in &mut ball_position {
        match *direction {
            BallDirection::Left => transform.translation.x -= BALL_VELOCITY * time.delta_seconds(),
            BallDirection::Right => transform.translation.x += BALL_VELOCITY * time.delta_seconds(),
        }

        if (transform.translation.x + (WINDOW_WIDTH / 2.0) - (BALL_RADIUS)) < 0.0 {
            *direction = BallDirection::Right
        } else if transform.translation.x > (WINDOW_WIDTH / 2.0 - BALL_RADIUS) {
            *direction = BallDirection::Left
        }
    }
}

fn setup_middle_line(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let number_lines: i32 =
        (WINDOW_HEIGHT / (MIDDLE_LINE_SEPARATOR_HEIGHT + MIDDLE_LINE_SEPARATOR_GAP_HEIGHT)) as i32;

    println!("number_lines: {}", number_lines);
    for i in 0..number_lines {
        commands.spawn((MaterialMesh2dBundle {
            mesh: meshes
                .add(
                    shape::Box::new(
                        MIDDLE_LINE_SEPARATOR_WIDTH,
                        MIDDLE_LINE_SEPARATOR_HEIGHT,
                        0.,
                    )
                    .into(),
                )
                .into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(
                0.,
                i as f32 * (MIDDLE_LINE_SEPARATOR_GAP_HEIGHT + MIDDLE_LINE_SEPARATOR_GAP_HEIGHT)
                    + MIDDLE_LINE_SEPARATOR_GAP_HEIGHT
                    - (WINDOW_HEIGHT / 2.0),
                0.,
            )),
            ..default()
        },));
    }
}

// TODO: Implement game score
#[derive(Component)]
struct Score {}
