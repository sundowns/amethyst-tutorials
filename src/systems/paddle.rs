extern crate amethyst;

mod pong {
    use amethyst::ecs::prelude::*;

    pub enum Side {
      Left,
      Right,
    }
    pub struct Paddle {
      pub side: Side,
    }
    impl Component for Paddle {
      type Storage = VecStorage<Self>;
    }

    pub const ARENA_HEIGHT: f32 = 100.0;
    pub const PADDLE_HEIGHT: f32 = 16.0;
}

use amethyst::core::Transform;
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;

// You'll have to mark PADDLE_HEIGHT as public in pong.rs
use pong::{Paddle, Side, ARENA_HEIGHT, PADDLE_HEIGHT};

pub struct PaddleSystem;

impl<'s> System<'s> for PaddleSystem {
  type SystemData = (
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Paddle>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (mut transforms, paddles, input): Self::SystemData) {
    for (paddle, transform) in (&paddles, &mut transforms).join() {
      let movement = match paddle.side {
        Side::Left => input.axis_value("left_paddle"),
        Side::Right => input.axis_value("right_paddle"),
      };
      if let Some(mv_amount) = movement {
        if mv_amount != 0.0 {
          let side_name = match paddle.side {
            Side::Left => "left",
            Side::Right => "right",
          };
          println!("Side {:?} moving {}", side_name, mv_amount);
        }
      }
    }
  }
}

fn main() {}