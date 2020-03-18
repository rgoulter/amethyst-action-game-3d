use amethyst;
use amethyst::{
    core::{timing::{Time}, Named, Parent},
    derive::SystemDesc,
    ecs::prelude::{Read, ReadStorage, System, SystemData},
    input::{InputHandler, StringBindings},
};

use crate::player::Player;

#[derive(Default, SystemDesc)]
pub struct DebugSystem;

impl<'a> System<'a> for DebugSystem {
    type SystemData = (
        Read<'a, InputHandler<StringBindings>>,
        Read<'a, Time>,
        ReadStorage<'a, Named>,
        ReadStorage<'a, Parent>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            _input,
            _time,
            _named,
            _parents,
            _players
        ) =
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
