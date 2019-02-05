use amethyst;

use amethyst::{
    core::{
        timing::{Time},
        Named,
        Parent,
    },
    ecs::prelude::{
        Read, ReadStorage, System,
    },
    renderer::{Material, MeshData,},
};

use crate::player::Player;

#[derive(Default)]
pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        Read<'a, Time>,
        ReadStorage<'a, Material>,
        ReadStorage<'a, MeshData>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (_time, _materials, _meshes, _named, _parents, _players) =
            data;

        // It's not that this is useless;
        //  but it's far too verbose.
        // Need to check for System Events or something
        //  to filter all the noise.
        
        // println!("==== Debug System ====");
        // println!("query: named AND material AND mesh_data");
        // for (name, _mat, _mesh) in (&named, &materials, &meshes).join() {
        //     println!("- found enitity (w/ name, material, mesh): {}", name.name);
        // }
        // println!("query: has_parent");
        // for (parent) in (&parents).join() {
        //     println!("- found enitity (w/ parent)");
        // }
        // println!("query: named AND has_parent");
        // for (name, parent) in (&named, &parents).join() {
        //     println!("- found enitity (w/ name, parent): {}", name.name);
        // }
        // println!("==== Debug System END ====");
    }
}
