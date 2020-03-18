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
