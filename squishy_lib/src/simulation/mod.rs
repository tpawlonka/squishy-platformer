mod components;

use bevy::asset::AssetContainer;
use bevy::color::palettes::basic::{BLUE, GRAY, YELLOW};
use bevy::math::vec3;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use bevy_rapier3d::rapier::prelude::DebugRenderObject::MultibodyJoint;
use crate::simulation::components::{SquishyCenter, SquishyNode};

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

    let y_offset = 5.0_f32;
    let scale = 0.1_f32;

    // commands.spawn((
    //     RigidBody::Dynamic,
    //     Mesh3d(meshes.add(Sphere::new(0.2))),
    //     MeshMaterial3d(materials.add(Color::Srgba(BLUE))),
    //     Collider::ball(0.2),
    //     Transform::from_xyz(0.0, -1.0 + y_offset, 0.0),
    //     ColliderMassProperties::Mass(10.0),
    //     Friction::coefficient(1.0),
    //     Restitution::coefficient(0.5),
    //     Sleeping::disabled(),
    //     Ccd::enabled(),
    // ));

    // test icosahedron
    // let coords = vec![
    //     [0.0, -1.0 + y_offset, -std::f32::consts::PHI],
    //     [0.0, 1.0 + y_offset, -std::f32::consts::PHI],
    //     [0.0, 1.0 + y_offset, std::f32::consts::PHI],
    //     [0.0, -1.0 + y_offset, std::f32::consts::PHI],
    //     [-1.0, -std::f32::consts::PHI + y_offset, 0.0],
    //     [1.0, -std::f32::consts::PHI + y_offset, 0.0],
    //     [1.0, std::f32::consts::PHI + y_offset, 0.0],
    //     [-1.0, std::f32::consts::PHI + y_offset, 0.0],
    //     [-std::f32::consts::PHI, 0.0 + y_offset, -1.0],
    //     [std::f32::consts::PHI, 0.0 + y_offset, -1.0],
    //     [std::f32::consts::PHI, 0.0 + y_offset, 1.0],
    //     [-std::f32::consts::PHI, 0.0 + y_offset, 1.0],
    //     [0.0, 0.0 + y_offset, 0.0],
    // ];

    let mut coords = spherical_to_cart(tile_unit_sphere_coords(8, 2, 1));
    coords.push(vec3(0.0, 0.0, 0.0));
    println!("{:?}", tile_unit_sphere_coords(8, 2, 1));
    println!("{:?}", &coords);
    let mut vertex = Vec::new();


    for coord in &coords {
        let id = commands.spawn(test(coord.x, coord.y+y_offset, coord.z)).id();
        vertex.push(id);
    }

    for i in 0..vertex.len() {
        let vert = vertex[i];
        for j in 0..coords.len() {
            if j == i {continue;}
            let dist = distance_euclid(coords[i], coords[j]);
            let joint = SpringJointBuilder::new(dist, 5.0, 5.0);
            let id = commands.spawn((
                ImpulseJoint::new(vertex[j], joint),
                )).id();
            commands.entity(vert).add_child(id);
        }
    }

    for id in &vertex {
        commands.entity(*id).insert((
                Mesh3d(meshes.add(Sphere::new(0.05))),
                MeshMaterial3d(materials.add(Color::Srgba(BLUE))),
            ));
    }
    commands.entity(vertex[vertex.len()-1]).insert(SquishyCenter);
}

fn distance_euclid(f1: Vec3, f2: Vec3) -> f32 {
    let mut result: f32 = 0.0;
    for i in 0..3 {
        result += (f1[i] - f2[i]).powi(2);
    }
    result.sqrt()
}



/// fractions must be even and a multiple of 4, incl_scale must be even
fn tile_unit_sphere_coords<'a>(fractions: u32, incl_scale: u32, azim_scale: u32) -> Vec<[f32; 3]> {
    let mut result = Vec::with_capacity(fractions.pow(2) as usize);
    let mut angle_az = 2.0 * std::f32::consts::PI / fractions as f32;
    let angle_incl = 2.0 * std::f32::consts::PI / (fractions * incl_scale) as f32;
    let radius = 1.0;
    let mut md = 0;
    let incl_step = fractions * incl_scale / 2;
    result.push([1.0, 0.0, 0.0]);
    result.push([1.0, std::f32::consts::PI, 0.0]);
    for i in 1..incl_step {
        let inclination = i as f32 * angle_incl;
        match i <= incl_step/2 {
            true => {md += 2*azim_scale}
            false => {md -= 2*azim_scale}
        };
        angle_az =  2.0 * std::f32::consts::PI / (fractions+md) as f32;
        for j in 0..fractions+md {
            let azimuth = j as f32 * angle_az;
            result.push([radius, inclination, azimuth]);
        }
    }
    result
}

/// c[0]: r         radius
/// c[1]: theta     inclination
/// c[2]: phi       azimuth
fn spherical_to_cart(coords: Vec<[f32; 3]>) -> Vec<Vec3> {
    let mut result = Vec::with_capacity(coords.len());
    coords.iter().for_each(|c| {
        result.push(Vec3::new(
            c[0]*c[1].sin()*c[2].cos(),
            c[0]*c[1].sin()*c[2].sin(),
            c[0]*c[1].cos()
        ));
    });
    result
}

fn test(x: f32, y: f32, z: f32) -> impl Bundle {
    let col = Group::GROUP_1;
    (
        RigidBody::Dynamic,
        Collider::ball(0.05),
        // Collider::cuboid(0.05, 0.05, 0.05),
        Transform::from_xyz(x, y, z),
        ColliderMassProperties::Mass(1.0),
        Friction::coefficient(100.0),
        Restitution::coefficient(5.0),
        Sleeping::disabled(),
        Ccd::enabled(),
        CollisionGroups::new(col, !col),
        LockedAxes::ROTATION_LOCKED,
        SquishyNode,
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