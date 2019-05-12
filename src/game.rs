mod player;
mod ball;

extern crate piston_window;
extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;

use piston_window::*;

use nalgebra::{Vector2, Isometry2};
use ncollide2d::shape::{Ball, ShapeHandle, Cuboid};
use nphysics2d::object::{ColliderDesc, RigidBodyDesc, BodyStatus};
use nphysics2d::world::World;
use nphysics2d::algebra::Velocity2;
use nphysics2d::material::{MaterialHandle, BasicMaterial};

pub struct Game {
    world: World<f64>,
    ball: Vec<ball::PongBall>,
    players: Vec<player::PongPlayer>,
}

impl Game {
    pub fn new() -> Game {
        let world : World<f64> = World::new();
        Game {
            world,
            ball: Vec::with_capacity(1),
            players: Vec::with_capacity(2),
        }
    }

    pub fn init(&mut self) {
        //TODO Write a ball init function specifically
        let ball_shape = ShapeHandle::new(Ball::new(ball::BALL_SIZE));
        let ball_collider = ColliderDesc::new(ball_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(2.0, 0.0)));

        let mut rb_desc = RigidBodyDesc::new()
            .collider(&ball_collider)
            .velocity(Velocity2::linear(ball::BALL_HORIZONTAL_SPEED, ball::BALL_VERTICAL_SPEED));

        let rigid_body = rb_desc.build(&mut self.world);
        let ball_handle = rigid_body.handle();

        let ball = ball::PongBall::new(ball_handle);
        self.ball.push(ball);

        //TODO Write a player init function specifically
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::repeat(3.0)));
        let player_collider = ColliderDesc::new(player_shape);
        let player_one_rb_desc = RigidBodyDesc::new()
            .collider(&player_collider);

        let player_two_rb_desc = RigidBodyDesc::new()
            .collider(&player_collider);

        let player_one_rigid_body = player_one_rb_desc
            .position(Isometry2::translation(100.0, 100.0))
            .status(BodyStatus::Kinematic)
            .build(&mut self.world);
        let player_one_handle = player_one_rigid_body.handle();

        let player_two_rigid_body = player_two_rb_desc
            .position(Isometry2::translation(300.0, 200.0))
            .status(BodyStatus::Kinematic)
            .build(&mut self.world);
        let player_two_handle = player_two_rigid_body.handle();

        let player_one = player::PongPlayer::new(player_one_handle);
        let player_two = player::PongPlayer::new(player_two_handle);

        self.players.push(player_one);
        self.players.push(player_two);

    }

    pub fn update(&mut self) {
        self.world.step();
    }

    pub fn render<G> (&self, context: Context, graphics: &mut G)
        where G: Graphics {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.ball[0].render(context, graphics, &self.world);

        for player in &self.players {
            player.render(context, graphics, &self.world);
        }
    }
}
