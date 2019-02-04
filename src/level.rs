use amethyst::{
    core::{
        nalgebra::{Translation3, UnitQuaternion, Vector3,},
        transform::{Transform},
        Parent,
    },
    ecs::prelude::{
        Entity, Write,
    },
    prelude::*,
    renderer::{
        AmbientColor, Camera, DirectionalLight, Light, MeshData,
        Projection, Rgba,
    },
};
use std::f32::consts::PI;

use crate::graphics::{
    Assets,
};
use crate::player::{
    init_player,
};

#[derive(Clone, Default)]
pub struct Level {
    player_location: Transform,
}

pub fn init_level(world: &mut World, assets: Assets, level: &Level) -> () {
    world.register::<MeshData>();
    init_grid(world, assets.clone());
    init_player(world, assets.clone(), level.player_location.clone());
    init_camera(world);
    init_lighting(world);
}

fn init_grid(world: &mut World, assets: Assets) -> () {
    init_checkerboard_grid(world, assets.clone());
    init_map_grid(world, assets.clone());
}

pub fn init_map_grid(world: &mut World, assets: Assets) -> () {
    let mut transform = Transform::default();
    transform.rotate_local(Vector3::x_axis(), -PI / 2.0);

    let _grid = world
        .create_entity()
        .with(transform)
        .with(assets.map_texture_material.clone())
        .with(assets.grid_of_sprites.clone())
        .build();
}

fn init_checkerboard_grid(world: &mut World, assets: Assets) -> Entity {
    let transform = Transform::default();

    let grid_root = world
        .create_entity()
        .with(transform)
        .build();

    let grid_num_rows = 8;
    let grid_num_cols = 8;

    for x in 0..grid_num_rows {
        for y in 0..grid_num_rows {
            if (x + y) % 2 == 1 {
                let mut transform = Transform::default();
                let tx = -0.5 + (x as f32 - (grid_num_rows / 2) as f32);
                let ty = 0.5 + (y as f32 - (grid_num_cols / 2) as f32);
                // println!("make grid at {}, {} for {}, {}", tx, ty, x, y);
                transform.set_xyz(tx, -0.1, ty);
                transform.rotate_local(Vector3::x_axis(), -PI / 2.0);

                let material = assets.grey_material.clone();
                let grid_mesh = assets.grid.clone();
                world
                    .create_entity()
                    .with(transform)
                    .with(grid_mesh)
                    .with(material)
                    .with(Parent { entity: grid_root })
                    .build();
            }
        }
    }

    grid_root
}

// fn init_camera(world: &mut World, parent: Entity) {
fn init_camera(world: &mut World) {
    // let position = Translation3::new(0.0, -20.0, 10.0);
    let position = Translation3::new(0.0, 15.0, 15.0);
    // let rotation = UnitQuaternion::from_euler_angles(0.7933533, 0.6087614, 0.0);
    // let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    // let rotation = UnitQuaternion::new_normalize(rotation);
    let rotation = UnitQuaternion::from_euler_angles(
        -PI / 4.0,
        0.0,
        0.0
    );

    let scale = Vector3::new(1.0, 1.0, 1.0);
    let transform = Transform::new(position, rotation, scale);
    world
        .create_entity()
        .with(Camera::from(Projection::perspective(
          1.3,
          1.0471975512,
        )))
        // .with(Parent { entity: parent })
        .with(transform)
        .build();
}

fn init_lighting(world: &mut World) {
    world.exec(
        |mut color: Write<'_, AmbientColor>| {
            color.0 = [1.0; 4].into();
        },
    );

    // let position = Translation3::new(15.0, 6.0, 0.0);
    // let rotation = Quaternion::new(0.7933533, 0.6087614, 0.0, 0.0);
    // let rotation = UnitQuaternion::new_normalize(rotation);
    // let scale = Vector3::new(1.0, 1.0, 1.0);
    // let transform = Transform::new(position, rotation, scale);

    let direction_light = DirectionalLight {
        color: Rgba(1.0, 1.0, 1.0, 1.0),
        direction: [-0.1, -0.1, 1.0]
    };
    world
        .create_entity()
        .with(Light::Directional(direction_light))
        .build();
}
