extern crate nalgebra;
extern crate nphysics2d;

use piston_window::*;

use nalgebra::Isometry2;
use nphysics2d::object::BodyHandle;
use nphysics2d::world::World;
use core::borrow::{Borrow, BorrowMut};

const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 50.0;

pub struct PongPlayer {
    pub shape: Rectangle,
    pub body: BodyHandle,
}

impl PongPlayer {
    pub fn render<G>(&self, context: Context, graphics: &mut G, world: &World<f64>)
        where G: Graphics {
        let player_body = world.rigid_body(self.body);
        match player_body {
            None => {},
            Some(b) => {
                let player_body = b.borrow();
                let pos = player_body.position().translation.vector;
                self.shape.draw(
                    [pos[0], pos[1], PLAYER_WIDTH, PLAYER_HEIGHT],
                    &context.draw_state,
                    context.transform,
                    graphics,
                )
            }
        }
    }

    pub fn new(body: BodyHandle) -> PongPlayer {
        PongPlayer {
            body,
            shape: Rectangle::new(BLACK),
        }
    }

    pub fn move_up(&mut self, world: &mut World<f64>) {
        let player_body = world.rigid_body_mut(self.body);
        match player_body {
            None => {},
            Some(b) => {
                let player_body = b.borrow_mut();
                let current_pos = player_body.position().translation.vector;
                player_body.set_position(Isometry2::translation(current_pos[0], current_pos[1] + 5.0))
            }
        }
    }

    pub fn move_down(&mut self, world: &mut World<f64>) {
        let player_body = world.rigid_body_mut(self.body);
        match player_body {
            None => {},
            Some(b) => {
                let player_body = b.borrow_mut();
                let current_pos = player_body.position().translation.vector;
                player_body.set_position(Isometry2::translation(current_pos[0], current_pos[1] - 5.0))
            }
        }
    }
}
