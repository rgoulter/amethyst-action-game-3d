use amethyst;

use amethyst::{
    core::{
        timing::Time,
        transform::{Transform},
    },
    ecs::prelude::{
        Join, Read, ReadStorage, System, WriteStorage
    },
    input::{
        InputHandler
    },
};
use std::f32::consts::PI;

use crate::game::*;

pub struct MovementSystem;

impl<'s> System<'s> for MovementSystem {
    type SystemData = (
        ReadStorage<'s, Player>,
        WriteStorage<'s, Transform>,
        Read<'s, InputHandler<String, String>>,
        Read<'s, Time>,
    );

    fn run(&mut self, (players, mut transforms, input, time): Self::SystemData) {
        let turn = input.axis_value("player_rotation").unwrap();
        let y_move = input.axis_value("player_acceleration").unwrap();

        let dt = time.delta_seconds();
        for (_, transform) in (&players, &mut transforms).join() {
            transform.move_forward(y_move as f32 * 2.0 * dt);
            // transform.move_local(Vector3::new(0.0, y_move as f32 * 2.0 * dt, 0.0));
            transform.yaw_local(turn as f32 * PI / 2.0 * dt);
            // transform.rotate_local(Vector3::z_axis(), turn as f32 * PI / 2.0 * dt);
        }
    }
}
