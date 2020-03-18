use std::borrow::Cow;
use std::collections::HashSet;

use amethyst::{
    core::{
        transform::Transform,
        Named,
    },
    ecs::{NullStorage},
    ecs::prelude::{
        Component, Entity,
    },
    prelude::*,
};

use crate::assets::Assets;
use crate::replace_material::ReplaceMaterial;
use crate::utils::print_gltf_info;

#[derive(Default)]
pub struct Player;

impl Component for Player {
    type Storage = NullStorage<Self>;
}

pub fn init_player(
    world: &mut World,
    assets: Assets,
    init_location: Transform
) -> Entity {
    let mut tank_replace_material_targets: HashSet<Cow<'static, str>> =
        HashSet::new();
    tank_replace_material_targets.insert(Cow::Borrowed("TankBase"));
    tank_replace_material_targets.insert(Cow::Borrowed("Turret"));
    tank_replace_material_targets.insert(Cow::Borrowed("TurretGun"));

    let replace_material = ReplaceMaterial {
        targets: tank_replace_material_targets,
        replacement: Some(assets.tank_blue_material.clone()),
    };

    let tank_gltf_mesh = assets.tank_gltf.clone();
    {
        let w : &mut World = world;
        print_gltf_info(w, &tank_gltf_mesh);
    }

    world
        .create_entity()
        .with(init_location)
        .with(Named { name: Cow::Borrowed("player_tank_replace") })
        .with(Player)
        .with(replace_material)
        .with(tank_gltf_mesh)
        .build()
}
