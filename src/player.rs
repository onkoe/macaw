use bevy::{
    // input::mouse::MouseMotion,
    input::mouse::MouseMotion,
    prelude::*,
    utils::Uuid,
    window::{CursorGrabMode, PrimaryWindow}, // window::{CursorGrabMode, PrimaryWindow},
};

use bevy_flycam::FlyCam;

// For entities that are players, such as in multiplayer
#[derive(Component)]
#[allow(unused)] // TODO: make it used!
pub struct Player {
    uuid: Uuid,
    username: String,
}

// Systems will handle logic, such as responding to input
// TODO: fix this shiii
pub fn player_input_system(
    mut mouse_input: EventReader<MouseMotion>,
    keyboard_input: Res<Input<KeyCode>>,
    mut keyboard_query: Query<&mut Transform, With<Player>>,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    let mut window = window_query.single_mut();

    const MOVEMENT_SPEED: f32 = 1.0; // was 0.2 lol

    for mut transform in keyboard_query.iter_mut() {
        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::W) {
            direction.z -= MOVEMENT_SPEED;
            tracing::debug!("w pressed, position: {}", transform.translation);
        }

        if keyboard_input.pressed(KeyCode::A) {
            direction.x -= MOVEMENT_SPEED;
            tracing::debug!("a pressed, position: {}", transform.translation);
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction.z += MOVEMENT_SPEED;
            tracing::debug!("s pressed, position: {}", transform.translation);
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction.x += MOVEMENT_SPEED;
            tracing::debug!("d pressed, position: {}", transform.translation);
        }

        if direction != Vec3::ZERO {
            direction = direction.normalize() * MOVEMENT_SPEED;
        }

        // fix movement lol
        let rotated_direction = transform.rotation.mul_vec3(direction);
        transform.translation += rotated_direction;

        if keyboard_input.pressed(KeyCode::ShiftLeft) {
            transform.translation.y -= 1.0;
            tracing::debug!("lshift pressed, position: {}", transform.translation);
        }

        if keyboard_input.pressed(KeyCode::Space) {
            transform.translation.y += MOVEMENT_SPEED;
            tracing::debug!("space pressed, position: {}", transform.translation);
        }

        if keyboard_input.just_pressed(KeyCode::Escape) {
            tracing::info!("Escape pressed. Displaying game menu!");
            match window.cursor.grab_mode {
                bevy::window::CursorGrabMode::None => {
                    // unpause
                    window.cursor.grab_mode = bevy::window::CursorGrabMode::Locked;
                    window.cursor.visible = false;
                }
                bevy::window::CursorGrabMode::Confined => panic!("mouse should never be confined"),
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
        }

        if window.cursor.grab_mode == CursorGrabMode::Locked {
            const SENS: (f32, f32) = (0.1, 0.1);

            let mut n_yaw: f32 = 0_f32; // up-down (y)
            let mut n_pitch: f32 = 0_f32; // left-right (x)

            for motion in mouse_input.read() {
                n_pitch += motion.delta.x * SENS.0;
                n_yaw -= motion.delta.y * SENS.1;
            }

            //Apply yaw rotation (around the y-axis)
            let yaw = Quat::from_rotation_y(-n_pitch.to_radians());
            transform.rotation = yaw * transform.rotation;

            //Apply pitch rotation (around the x-axis)
            //Calculate the current pitch to constrain the camera from flipping over
            let current_pitch: f32 = transform.rotation.to_euler(EulerRot::XYZ).1;
            let new_pitch = (current_pitch + n_yaw.to_radians())
                .clamp(-std::f32::consts::FRAC_PI_2, std::f32::consts::FRAC_PI_2);
            let pitch_delta = new_pitch - current_pitch;
            let pitch = Quat::from_rotation_x(pitch_delta);
            transform.rotation *= pitch;
        }
    }
}

pub fn setup(mut commands: Commands) {
    // Spawning a player entity with components
    commands
        .spawn((
            Player {
                uuid: Uuid::new_v4(),
                username: "Player".to_owned(),
            },
            Transform::from_xyz(10.0, 12.0, 16.0),
            GlobalTransform::default(),
        ))
        .insert(Name::new("Player"))
        // Add a camera to the player or as a separate entity as needed
        .with_children(|parent: &mut ChildBuilder<'_, '_, '_>| {
            parent.spawn((Camera3dBundle::default(), FlyCam));
        });
}
