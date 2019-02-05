use std::borrow::Cow;
use std::collections::HashSet;

use amethyst::{
    core::{
        transform::{Transform},
        Named,
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
use crate::replace_material::ReplaceMaterial;
use crate::utils::print_gltf_info;

#[derive(Default)]
pub struct UndergroundBase;

impl Component for UndergroundBase {
    type Storage = NullStorage<Self>;
}

pub fn init_underground_base(
    world: &mut World,
    assets: Assets,
    init_location: Transform
) -> Entity {
    let underground_base = world
        .create_entity()
        .with(init_location)
        .with(UndergroundBase)
        .build();

    let mut model_transform = Transform::default();
    model_transform.yaw_local(PI); // The GLTF model needs to be turned around.
    let _model_rotation = world
        .create_entity()
        .with(Parent { entity: underground_base })
        // .with(Named { name: Cow::Borrowed("player_tank_replace") })
        // .with(replace_material)
        .with(model_transform)
        .build();

    // front of the model in Blender is -Y;
    // but the GLTF export has it as +Z.
    let gltf_mesh = assets.underground_base.unanimated.clone();
    let _gltf_entity = world
        .create_entity()
        .with(gltf_mesh)
        .with(Parent { entity: _model_rotation })
        .build();

    underground_base
}
