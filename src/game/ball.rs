extern crate ncollide2d;
extern crate nphysics2d;

use piston_window::*;

use core::borrow::Borrow;
use nphysics2d::object::BodyHandle;
use nphysics2d::world::World;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

pub const BALL_SIZE: f64 = 20.0;
pub const BALL_HORIZONTAL_SPEED: f64 = 18.0;
pub const BALL_VERTICAL_SPEED: f64 = 10.0;

pub struct PongBall {
    shape: Ellipse,
    body: BodyHandle,
}

impl PongBall {
    pub fn new(body: BodyHandle) -> PongBall {
        PongBall {
            shape: Ellipse::new(BLACK),
            body,
        }
    }

    pub fn render<G>(&self, context: Context, graphics: &mut G, world: &World<f64>)
    where
        G: Graphics,
    {
        let body = world.rigid_body(self.body);
        match body {
            None => {}
            Some(b) => {
                let ball_body = b.borrow();
                let pos = ball_body.position().translation.vector;
                self.shape.draw(
                    [pos[0], pos[1], BALL_SIZE, BALL_SIZE],
                    &context.draw_state,
                    context.transform,
                    graphics,
                );
            }
        }
    }
}
