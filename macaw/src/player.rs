use bevy::{
    input::mouse::MouseMotion,
    prelude::*,
    utils::Uuid,
    window::{CursorGrabMode, PrimaryWindow},
};

use crate::{loader::LoaderState, renderer::skybox};

#[derive(Component)]
pub struct PlayerCamera;

pub struct MacawPlayerPlugin;

impl Plugin for MacawPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Player::setup);

        // add skybox when we finish loading the game
        app.add_systems(
            OnExit::<LoaderState>(LoaderState::Loading),
            skybox::add_skybox.run_if(run_once()),
        );

        app.add_systems(Update, Player::player_input_system);
    }
}

// For entities that are players, such as in multiplayer
#[derive(Component)]
#[allow(unused)] // TODO: make it used!
pub struct Player {
    uuid: Uuid,
    username: String,
}

impl Player {
    pub fn setup(mut commands: Commands) {
        // TODO?: use
        let _player = commands
            .spawn((
                Player {
                    uuid: Uuid::new_v4(),
                    username: "Player".to_owned(),
                },
                Transform::from_xyz(0.0, 17.0, 10.0),
                GlobalTransform::default(),
            ))
            .insert(Name::new("Player"))
            .with_children(|parent: &mut ChildBuilder<'_>| {
                parent.spawn(Camera3dBundle::default()).insert(PlayerCamera);
            })
            .id();
    }

    /// Controls the player using application input.
    pub fn player_input_system(
        mut mouse_input: EventReader<MouseMotion>,
        keyboard_input: Res<ButtonInput<KeyCode>>,
        mut keyboard_query: Query<&mut Transform, With<Player>>,
        mut window_query: Query<&mut Window, With<PrimaryWindow>>,
        time: Res<Time>,
    ) {
        // TODO: fix this giant method lol

        let mut window = window_query.single_mut();

        for mut transform in keyboard_query.iter_mut() {
            let mut direction = Vec3::ZERO;

            fn get_movement_speed(keyboard_input: &Res<ButtonInput<KeyCode>>) -> f32 {
                if keyboard_input.pressed(KeyCode::ControlLeft) {
                    1.0
                } else {
                    0.05
                }
            }

            if keyboard_input.pressed(KeyCode::KeyW) {
                direction.z -= time.delta_seconds() / get_movement_speed(&keyboard_input);
            }

            if keyboard_input.pressed(KeyCode::KeyA) {
                direction.x -= time.delta_seconds() / get_movement_speed(&keyboard_input);
            }

            if keyboard_input.pressed(KeyCode::KeyS) {
                direction.z += time.delta_seconds() / get_movement_speed(&keyboard_input);
            }

            if keyboard_input.pressed(KeyCode::KeyD) {
                direction.x += time.delta_seconds() / get_movement_speed(&keyboard_input);
            }

            if direction != Vec3::ZERO {
                direction = direction.normalize() * get_movement_speed(&keyboard_input);
            }

            // fix movement lol
            let rotated_direction = transform.rotation.mul_vec3(direction);
            transform.translation += rotated_direction;

            if keyboard_input.pressed(KeyCode::ShiftLeft) {
                transform.translation.y -=
                    time.delta_seconds() / (get_movement_speed(&keyboard_input) / 5.0);
            }

            if keyboard_input.pressed(KeyCode::Space) {
                transform.translation.y +=
                    time.delta_seconds() / (get_movement_speed(&keyboard_input) / 5.0);
            }

            if keyboard_input.just_pressed(KeyCode::Escape) {
                match window.cursor.grab_mode {
                    bevy::window::CursorGrabMode::None => {
                        // unpause
                        window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
                        window.cursor.visible = false;
                    }
                    bevy::window::CursorGrabMode::Confined => {
                        panic!("mouse should never be confined")
                    }
                    bevy::window::CursorGrabMode::Locked => {
                        // pause
                        window.cursor.grab_mode = bevy::window::CursorGrabMode::None;
                        window.cursor.visible = true;
                    }
                }
            }

            if keyboard_input.just_pressed(KeyCode::Slash) {
                tracing::info!("Slash pressed. Displaying commands!");

                // TODO: actually do that

                transform.translation = Default::default();
                transform.rotation = Default::default();
            }

            if window.cursor.grab_mode == CursorGrabMode::Locked {
                const SENS: (f32, f32) = (0.1, 0.1);

                let mut n_yaw: f32 = 0_f32; // up-down (y)
                let mut n_pitch: f32 = 0_f32; // left-right (x)

                for motion in mouse_input.read() {
                    n_pitch += motion.delta.x * SENS.0;
                    n_yaw -= motion.delta.y * SENS.1;
                }

                let yaw = Quat::from_rotation_y(-n_pitch.to_radians());
                transform.rotation = yaw * transform.rotation;

                let current_pitch: f32 = transform.rotation.to_euler(EulerRot::XYZ).1;
                let new_pitch = (current_pitch + n_yaw.to_radians())
                    .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
                let pitch_delta = new_pitch - current_pitch;
                let pitch = Quat::from_rotation_x(pitch_delta);
                transform.rotation *= pitch;
            }
        }
    }
}
