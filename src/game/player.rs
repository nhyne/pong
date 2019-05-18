extern crate nalgebra;
extern crate nphysics2d;

use piston_window::*;

use core::borrow::{Borrow, BorrowMut};
use nalgebra::Isometry2;
use nphysics2d::object::BodyHandle;
use nphysics2d::world::World;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 50.0;
pub const PLAYER_SPEED: f64 = 5.0;

pub struct PongPlayer {
    pub shape: Rectangle,
    pub body: BodyHandle,
}

impl PongPlayer {
    pub fn render<G: Graphics>(&self, context: Context, graphics: &mut G, world: &World<f64>) {
        let player_body = world.rigid_body(self.body);
        match player_body {
            None => {}
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
            None => {}
            Some(b) => {
                let player_body = b.borrow_mut();
                let current_pos = player_body.position().translation.vector;
                player_body.set_position(Isometry2::translation(
                    current_pos[0],
                    current_pos[1] - PLAYER_SPEED,
                ))
            }
        }
    }

    pub fn move_down(&mut self, world: &mut World<f64>) {
        let player_body = world.rigid_body_mut(self.body);
        match player_body {
            None => {}
            Some(b) => {
                let player_body = b.borrow_mut();
                let current_pos = player_body.position().translation.vector;
                player_body.set_position(Isometry2::translation(
                    current_pos[0],
                    current_pos[1] + PLAYER_SPEED,
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::game::player::{PongPlayer, PLAYER_SPEED};
    use nalgebra::Vector2;
    use ncollide2d::shape::{Cuboid, ShapeHandle};
    use nphysics2d::object::{BodyStatus, ColliderDesc, RigidBodyDesc};
    use nphysics2d::world::World;

    fn init_player(world: &mut World<f64>) -> PongPlayer {
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::new(7.5, 25.0)));
        let player_collider = ColliderDesc::new(player_shape);
        let player_rb_desc = RigidBodyDesc::new().collider(&player_collider);

        let player_rigid_body = player_rb_desc.status(BodyStatus::Kinematic).build(world);
        let player_handle = player_rigid_body.handle();

        PongPlayer::new(player_handle)
    }

    #[test]
    fn player_move_down() {
        let mut world = World::new();
        let mut player = init_player(&mut world);

        let initial_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        player.move_down(&mut world);

        let new_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        assert_eq!(initial_pos + PLAYER_SPEED, new_pos);
    }

    #[test]
    fn player_move_up() {
        let mut world = World::new();
        let mut player = init_player(&mut world);

        let initial_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        player.move_up(&mut world);

        let new_pos = if let Some(body) = world.rigid_body(player.body) {
            body.position().translation.vector[1]
        } else {
            0.0
        };

        assert_eq!(initial_pos - PLAYER_SPEED, new_pos);
    }
}
