use std::time::Duration;

use bevy::{prelude::*, time::common_conditions::on_timer};
use bevy_octopus::{prelude::*, transports::tcp::TcpAddress};

use crate::shared::*;
extern crate shared;

fn main() {
    let mut app = App::new();

    shared_setup(&mut app);

    app.add_transformer::<PlayerInformation, JsonTransformer>(JSON_CHANNEL)
        .add_transformer::<PlayerInformation, BincodeTransformer>(BINCODE_CHANNEL)
        .add_systems(Startup, setup_clients)
        .add_systems(
            Update,
            (
                client_send_raw_message_to_channel::<TcpAddress>,
                send_json_message,
                send_bincode_message,
            )
                .run_if(on_timer(Duration::from_secs_f64(1.0))),
        )
        .add_systems(Update, (handle_raw_packet, handle_message_events))
        .run();
}

/// set up a simple 3D scene
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // circular base
    commands.spawn(PbrBundle {
        mesh: meshes.add(Circle::new(4.0)),
        material: materials.add(Color::WHITE),
        transform: Transform::from_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // cube
    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0.0, 0.5, 0.0),
        ..default()
    });
    // light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn spawn_player (
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

){
    commands.spawn(PbrBundle {
        mesh: meshes.add(Capsule3d::new(1.0, 1.0)),
        material: materials.add(Color::srgb_u8(124, 0, 100)),
        transform: Transform::from_xyz(2.0, 2.5, 3.0),
        ..default()
    }); 
}
//bevy octopus
fn setup_clients(mut commands: Commands) {
    commands.spawn((
        NetworkBundle::new(RAW_CHANNEL),
        ClientNode(TcpAddress::new("127.0.0.1:5003")),
    ));
    commands.spawn((
        NetworkBundle::new(JSON_CHANNEL),
        ClientNode(TcpAddress::new("127.0.0.1:5004")),
    ));
    commands.spawn((
        NetworkBundle::new(BINCODE_CHANNEL),
        ClientNode(TcpAddress::new("127.0.0.1:5005")),
    ));
}