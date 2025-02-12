use bevy::asset::AssetContainer;
use bevy::color::palettes::basic::{BLUE, GRAY, YELLOW};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::DebugRenderObject::MultibodyJoint;

pub struct SimulationPlugin;

impl Plugin for SimulationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, (setup_level, setup_character));
    }
}

fn setup_character(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // let test = commands.spawn((
    //     RigidBody::Dynamic,
    //     Mesh3d(meshes.add(Sphere::new(0.2))),
    //     MeshMaterial3d(materials.add(Color::Srgba(BLUE))),
    //     Collider::ball(0.2),
    //     Transform::from_xyz(0.0, 5.0, 0.0),
    //     ColliderMassProperties::Mass(10.0),
    //     Friction::coefficient(1.0),
    //     Restitution::coefficient(0.5),
    //     Sleeping::disabled(),
    //     Ccd::enabled(),
    // )).id();

    let y_offset = 5.0_f32;
    let scale = 0.1_f32;

    commands.spawn((
        RigidBody::Dynamic,
        Mesh3d(meshes.add(Sphere::new(0.2))),
        MeshMaterial3d(materials.add(Color::Srgba(BLUE))),
        Collider::ball(0.2),
        Transform::from_xyz(0.0, -1.0 + y_offset, 0.0),
        ColliderMassProperties::Mass(10.0),
        Friction::coefficient(1.0),
        Restitution::coefficient(0.5),
        Sleeping::disabled(),
        Ccd::enabled(),
    ));

    let coords = vec![
        [0.0, -1.0 + y_offset, -std::f32::consts::PHI],
        [0.0, 1.0 + y_offset, -std::f32::consts::PHI],
        [0.0, 1.0 + y_offset, std::f32::consts::PHI],
        [0.0, -1.0 + y_offset, std::f32::consts::PHI],
        [-1.0, -std::f32::consts::PHI + y_offset, 0.0],
        [1.0, -std::f32::consts::PHI + y_offset, 0.0],
        [1.0, std::f32::consts::PHI + y_offset, 0.0],
        [-1.0, std::f32::consts::PHI + y_offset, 0.0],
        [-std::f32::consts::PHI, 0.0 + y_offset, -1.0],
        [std::f32::consts::PHI, 0.0 + y_offset, -1.0],
        [std::f32::consts::PHI, 0.0 + y_offset, 1.0],
        [-std::f32::consts::PHI, 0.0 + y_offset, 1.0],
    ];

    let mut vertex = Vec::new();


    for coord in &coords {
        let id = commands.spawn(test(coord[0], coord[1], coord[2])).id();
        vertex.push(id);
    }

    for i in 0..vertex.len() {
        let vert = vertex[i];
        for j in 0..coords.len() {
            if j == i {continue;}
            let dist = distance_euclid(&coords[i][..], &coords[j][..]);
            let joint = SpringJointBuilder::new(dist*100.0, 20.0, 10.0);
            let id = commands.spawn((
                ImpulseJoint::new(vertex[j], joint),
                )).id();
            commands.entity(vert).add_child(id);
        }
    }
}

pub fn distance_euclid(f1: &[f32], f2: &[f32]) -> f32 {
    let mut result: f32 = 0.0;
    for i in 0..f1.len() {
        result += (f1[i] - f2[i]).powi(2);
    }
    result.powi(-2)
}

fn test(x: f32, y: f32, z: f32) -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::ball(0.2),
        Transform::from_xyz(x, y, z),
        ColliderMassProperties::Mass(1.0),
        Friction::coefficient(1000.0),
        Restitution::coefficient(0.5),
        Sleeping::disabled(),
        Ccd::enabled(),
    )
}

fn test2(x: f32, y: f32, z: f32) -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::ball(0.1),
        Transform::from_xyz(x, y, z),
        ColliderMassProperties::Mass(10.0),
        Friction::coefficient(1.0),
        Restitution::coefficient(0.5),
        Sleeping::disabled(),
        Ccd::enabled(),
    )
}

fn setup_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.insert_resource(AmbientLight {
        color: Color::WHITE,
        brightness: 1000.0,
    });


    let ground_size = 200.0;
    let ground_height = 0.1;

    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(ground_size, ground_height, ground_size))),
        MeshMaterial3d(materials.add(Color::Srgba(GRAY))),
        Collider::cuboid(ground_size, ground_height, ground_size),
        Transform::from_xyz(0.0, -ground_height, 0.0),
        RigidBody::Fixed
    ));

    let obstacle_size = 60.0;
    let obstacle = Cuboid::new(obstacle_size, obstacle_size, obstacle_size);

    commands.spawn((
        Mesh3d(meshes.add(obstacle)),
        MeshMaterial3d(materials.add(Color::Srgba(YELLOW))),
        Collider::cuboid(obstacle_size/2.0, obstacle_size/2.0, obstacle_size/2.0),
        Transform {
            translation: vec3(-100.0, -20.0, 40.0),
            rotation: Quat::from_rotation_y(-60f32.to_radians()) * Quat::from_rotation_x(60f32.to_radians()),
            ..default()
        },
        Friction::new(1.0),
        RigidBody::Fixed
    ));

}