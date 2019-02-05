use amethyst;

use amethyst::{
    animation::{
        AnimationCommand, AnimationControlSet, AnimationSet,
        EndControl,
        get_animation_set,
    },
    core::{
        transform::{Transform},
        Parent,
    },
    ecs::prelude::{
        Entity, Join, Read, ReadStorage, System, WriteStorage
    },
    input::{
        InputHandler
    },
    renderer::{
        VirtualKeyCode,
    },
};

use crate::underground_base::UndergroundBase;

fn start_animation(
    gltf_main_entity: Entity,
    animation_sets: &ReadStorage<'_, AnimationSet<usize, Transform>>,
    animation_control_sets:
        &mut WriteStorage<'_, AnimationControlSet<usize, Transform>>,
) {
    if let Some(animation_set) = animation_sets.get(gltf_main_entity) {
        let animation_control_set =
            get_animation_set::<usize, Transform>(
                animation_control_sets,
                gltf_main_entity
            ).unwrap(); // smell

        // AFAICT, Blender => GLTF => Amethyst
        //  maps Blender scene to GLTF scene,
        //  and Blender channels to animations in the animation set.
        // So, "Run (all the channels) a Blender scene" is like
        //  "run all the animations in the animation set".
        for (animation_index, animation) in &animation_set.animations {
            animation_control_set.add_animation(
                *animation_index,
                &animation,
                EndControl::Stay,
                1.0,
                AnimationCommand::Start,
            );
        }
    }
}

pub struct UndergroundBaseControlSystem;

impl<'s> System<'s> for UndergroundBaseControlSystem {
    type SystemData = (
        Read<'s, InputHandler<String, String>>,
        WriteStorage<'s, UndergroundBase>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, AnimationSet<usize, Transform>>,
        WriteStorage<'s, AnimationControlSet<usize, Transform>>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (input, mut bases, parents, animations, mut sets) = data;

        if input.key_is_down(VirtualKeyCode::J) {
            for (_base, parent) in (&mut bases, &parents).join() {
                let gltf_main_entity = parent.entity;
                let animation_control_set =
                    get_animation_set::<usize, Transform>(
                        &mut sets,
                        gltf_main_entity
                    );
                if let Some(animation_control_set) = animation_control_set {
                    if animation_control_set.is_empty() {
                        start_animation(
                            gltf_main_entity,
                            &animations,
                            &mut sets
                        );
                    }
                } else {
                    println!("Underground Base Control: unable to find animation control set for entity!");
                }
            }
        }
    }
}
