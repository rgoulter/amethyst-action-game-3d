use amethyst::{
    core::{
        transform::{Transform},
        Parent,
    },
    ecs::{NullStorage},
    ecs::prelude::{
        Component, Entity,
    },
    prelude::*,
};
use std::f32::consts::PI;

use crate::graphics::Assets;
use crate::utils::print_gltf_info;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

pub fn init_player(world: &mut World, assets: Assets, init_location: Transform) -> Entity {
    let player = world
        .create_entity()
        .with(init_location)
        .with(Player)
        .build();

    let mut tank_transform = Transform::default();
    tank_transform.yaw_local(PI); // <-- this should turn the model around
    let _model_rotation = world
        .create_entity()
        .with(Parent { entity: player })
        .with(tank_transform)
        .build();

    // front of the model in Blender is -Y;
    // but the GLTF export has it as +Z.
    let tank_gltf_mesh = assets.tank_gltf.clone();
    {
        let w : &mut World = world;
        print_gltf_info(w, &tank_gltf_mesh);
    }
    let _tank_entity = world
        .create_entity()
        .with(tank_gltf_mesh)
        .with(Parent { entity: _model_rotation })
        .build();

    player
}
