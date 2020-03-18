use std::f32::consts::PI;

use amethyst;
use amethyst::{
    core::{
        timing::Time,
        transform::{Transform},
    },
    derive::SystemDesc,
    ecs::prelude::{
        Join, Read, ReadStorage, System, WriteStorage
    },
    input::{InputHandler, StringBindings},
};
use amethyst::ecs::SystemData;
use nalgebra::Vector3;

use crate::player::Player;

#[derive(SystemDesc)]
pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut transforms, input, time): Self::SystemData) {
        let turn = input.axis_value("player_rotation").unwrap_or_else(|| 0.0);
        let z_move = input.axis_value("player_acceleration").unwrap_or_else(|| 0.0);

        let dt = time.delta_seconds();
        for (_, transform) in (&players, &mut transforms).join() {
            let delta_z = z_move as f32 * 2.0 * dt;
            transform.append_translation(Vector3::new(0.0, 0.0, delta_z));
            transform.append_rotation_y_axis(turn as f32 * PI / 2.0 * dt); // <-- no yaw_local?
        }
    }
}
