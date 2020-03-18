use std::f32::consts::PI;

use amethyst::{
    core::transform::Transform,
    ecs::prelude::Write,
    prelude::*,
    renderer::{
        camera::Projection,
        light::{
            DirectionalLight,
            Light,
        },
        resources::AmbientColor,
        palette::{Srgb, Srgba},
        Camera,
    },
};
use nalgebra::{Translation3, UnitQuaternion, Vector3,};

use crate::assets::Assets;
use crate::player::init_player;

#[derive(Clone, Default)]
pub struct Level {
    player_location: Transform,
}

pub fn init_level(world: &mut World, assets: Assets, level: &Level) -> () {
    init_grid(world, assets.clone());
    init_player(world, assets.clone(), level.player_location.clone());
    init_camera(world);
    init_lighting(world);
}

fn init_grid(world: &mut World, assets: Assets) -> () {
    // init_checkerboard_grid(world, assets.clone());
    init_map_grid(world, assets.clone());
}

pub fn init_map_grid(world: &mut World, assets: Assets) -> () {
    let mut transform = Transform::default();
    transform.set_rotation_euler(-PI / 2.0, 0.0, 0.0);

    let _grid = world
        .create_entity()
        .with(transform)
        .with(assets.map_texture_material.clone())
        .with(assets.sprite_grid.clone())
        .build();
}

// fn init_checkerboard_grid(world: &mut World, assets: Assets) -> Entity {
    // let transform = Transform::default();

    // let grid_root = world
        // .create_entity()
        // .with(transform)
        // .build();

    // let grid_num_rows = 8;
    // let grid_num_cols = 8;

    // for x in 0..grid_num_rows {
        // for y in 0..grid_num_rows {
            // if (x + y) % 2 == 1 {
                // let mut transform = Transform::default();
                // let tx = -0.5 + (x as f32 - (grid_num_rows / 2) as f32);
                // let ty = 0.5 + (y as f32 - (grid_num_cols / 2) as f32);
                // // println!("make grid at {}, {} for {}, {}", tx, ty, x, y);
                // transform.set_xyz(tx, -0.1, ty);
                // transform.rotate_local(Vector3::x_axis(), -PI / 2.0);

                // let material = assets.grey_material.clone();
                // let grid_mesh = assets.grid.clone();
                // world
                    // .create_entity()
                    // .with(transform)
                    // .with(grid_mesh)
                    // .with(material)
                    // .with(Parent { entity: grid_root })
                    // .build();
            // }
        // }
    // }

    // grid_root
// }

fn init_camera(world: &mut World) {
    let position = Translation3::new(0.0, 15.0, 15.0);
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
          0.01,
          1024.0,
        )))
        .with(transform)
        .build();
}

fn init_lighting(world: &mut World) {
    world.exec(
        |mut color: Write<'_, AmbientColor>| {
            color.0 = Srgba::new(1.0, 1.0, 1.0, 1.0);
        },
    );

    let direction_light = DirectionalLight {
        color: Srgb::new(1.0, 1.0, 1.0),
        direction: [-0.1, -0.1, 1.0].into(),
        intensity: 1.0
    };
    world
        .create_entity()
        .with(Light::Directional(direction_light))
        .build();
}
