use bevy::asset::AssetContainer;
use bevy::color::palettes::basic::{BLUE, GRAY, YELLOW};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

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
    //
    // let joint = SpringJointBuilder::new(1.0, 1.0, 1.0)
    //     .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
    //     .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
    //
    // let test2 = commands.spawn((
    //     RigidBody::Dynamic,
    //     Mesh3d(meshes.add(Sphere::new(0.2))),
    //     MeshMaterial3d(materials.add(Color::Srgba(BLUE))),
    //     Collider::ball(0.2),
    //     Transform::from_xyz(1.0, 5.0, 0.0),
    //     ColliderMassProperties::Mass(10.0),
    //     Friction::coefficient(1.0),
    //     Restitution::coefficient(0.5),
    //     Sleeping::disabled(),
    //     Ccd::enabled(),
    //     ImpulseJoint::new(test, joint),
    // )).id();
    // let z_offset = 10.0_f32;
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
        (0.0, 1.0 + y_offset, -std::f32::consts::PHI),
        (0.0, 1.0 + y_offset, std::f32::consts::PHI),
        (0.0, -1.0 + y_offset, std::f32::consts::PHI),
        (-1.0, -std::f32::consts::PHI + y_offset, 0.0),
        (1.0, -std::f32::consts::PHI + y_offset, 0.0),
        (1.0, std::f32::consts::PHI + y_offset, 0.0),
        (-1.0, std::f32::consts::PHI + y_offset, 0.0),
        (-std::f32::consts::PHI, 0.0 + y_offset, -1.0),
        (std::f32::consts::PHI, 0.0 + y_offset, -1.0),
        (std::f32::consts::PHI, 0.0 + y_offset, 1.0),
        (-std::f32::consts::PHI, 0.0 + y_offset, 1.0),
    ];

    let mut vertex = Vec::new();
    let mut vertex2 = Vec::<Vec<Entity>>::new();
    for i in 0..12 {
        vertex2.push(Vec::new());
    }

    for coord in coords {
        let id = commands.spawn(test(coord.0, coord.1, coord.2)).id();
        for i in 0..12 {
            let idx = commands.spawn(test2(coord.0, coord.1, coord.2)).id();
            let joint = FixedJointBuilder::new()
                .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
                .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
            commands.entity(idx).insert(MultibodyJoint::new(id, TypedJoint::from(joint)));
            vertex2[i].push(idx);
        }
        vertex.push(id);
    }


    // vertex.push(commands.spawn(test(0.0, -1.0 + y_offset, -std::f32::consts::PHI)).id());
    // for i in 0..12 {
    //     let id = commands.spawn(test2(0.0, -1.0 + y_offset, -std::f32::consts::PHI)).id();
    //     let joint = FixedJointBuilder::new()
    //         .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
    //         .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
    //     commands.entity(id).insert(MultibodyJoint::new(vertex[0], TypedJoint::from(joint)));
    //     vertex2[i].push(id);
    // }


    // for i in 0..12 {
    //     let vert = vertex[i];
    //     println!("vert {:?}", i);
    //     for j in 0..12 {
    //         if i == j { continue; }
    //         let vert2 = vertex[j];
    //         println!("vert2 {:?}", j);
    //         let joint = SpringJointBuilder::new(1.0, 100.0, 1000.0)
    //             .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
    //             .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
    //         commands.entity(vert).insert(ImpulseJoint::new(vert2, joint));
    //     }
    // }

    // for vert in &vertex {
    //     for vert2 in &vertex {
    //         if *vert == *vert2 { continue; }
    //         let joint = SpringJointBuilder::new(1.0, 1.0, 1.0)
    //             .local_anchor1(Vec3::new(0.0, 0.0, 0.0))
    //             .local_anchor2(Vec3::new(0.0, 0.0, 0.0));
    //         // vert.insert(ImpulseJoint::new(*vert2, joint))
    //         commands.entity(*vert).insert(ImpulseJoint::new(*vert2, joint));
    //     }
    // }
}

fn test(x: f32, y: f32, z: f32) -> impl Bundle {
    (
        RigidBody::Dynamic,
        Collider::ball(0.2),
        Transform::from_xyz(x, y, z),
        ColliderMassProperties::Mass(10.0),
        Friction::coefficient(1.0),
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