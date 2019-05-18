// use amethyst::core::Transform;
// use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
// use amethyst::input::InputHandler;

// use crate::pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

// pub struct TankSystem;

// pub const MOVEMENT_SCALAR: f32 = 1.4;

// impl<'s> System<'s> for TankSystem {
//     type SystemData = (
//         WriteStorage<'s, Transform>,
//         ReadStorage<'s, Paddle>,
//         Read<'s, InputHandler<String, String>>,
//     );

//     fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
//         for (paddle, transform) in (&paddles, &mut transforms).join() {
//             let movement = match paddle.side {
//                 Side::Left => input.axis_value("left_paddle"),
//                 Side::Right => input.axis_value("right_paddle"),
//             };

//             if let Some(mv_amount) = movement {
//                 let scaled_amount = MOVEMENT_SCALAR * mv_amount as f32;
//                 let paddle_y = transform.translation().y;
//                 transform.set_y(
//                     (paddle_y + scaled_amount)
//                         .min(ARENA_HEIGHT - PADDLE_HEIGHT * 0.5)
//                         .max(PADDLE_HEIGHT * 0.5),
//                 );
//             }
//         }
//     }
// }