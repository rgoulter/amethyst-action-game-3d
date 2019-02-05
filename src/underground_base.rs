use amethyst::{
    core::{
        transform::{Transform},
        Parent,
    },
    ecs::{DenseVecStorage},
    ecs::prelude::{
        Component, Entity,
    },
    prelude::*,
};
use std::f32::consts::PI;

use crate::graphics::Assets;

#[derive(Default)]
pub struct UndergroundBase;

impl Component for UndergroundBase {
    type Storage = DenseVecStorage<Self>;
}

pub fn init_underground_base(
    world: &mut World,
    assets: Assets,
    init_location: Transform
) -> Entity {
    let mut model_transform = init_location.clone();
    model_transform.yaw_local(PI); // The GLTF model needs to be turned around.
    let model_rotation = world
        .create_entity()
        .with(model_transform)
        .build();

    // front of the model in Blender is -Y;
    // but the GLTF export has it as +Z.
    let gltf_mesh = assets.underground_base.top_right.clone();
    let gltf_entity = world
        .create_entity()
        .with(gltf_mesh)
        .with(Parent { entity: model_rotation })
        .build();

    let _underground_base = world
        .create_entity()
        .with(Parent { entity: gltf_entity })
        .with(UndergroundBase)
        .build();

    model_rotation
}
