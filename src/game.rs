mod ball;
mod player;

extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate piston_window;

use piston_window::*;

use nalgebra::{Isometry2, Vector2};
use ncollide2d::shape::{Ball, Cuboid, ShapeHandle};
use nphysics2d::algebra::Velocity2;
use nphysics2d::material::{BasicMaterial, MaterialHandle};
use nphysics2d::object::{BodyStatus, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::World;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const WALL_DRAW_LENGTH: i32 = 800;
const WALL_DRAW_HEIGHT: i32 = 20;
const WALL_BODY_LENGTH: i32 = 800;
const WALL_BODY_HEIGHT: i32 = 1;

const TOP_WALL_X_POSITION: i32 = 0;
const TOP_WALL_Y_POSITION: i32 = 0;
const BOTTOM_WALL_X_POSITION: i32 = 0;
const BOTTOM_WALL_Y_POSITION: i32 = 400;

pub struct Game {
    pub world: World<f32>,
    //TODO these really don't need to be Vecs
    ball: Vec<ball::PongBall>,
    players: Vec<player::PongPlayer>,
}

impl Game {
    pub fn new() -> Game {
        let world: World<f32> = World::new();
        Game {
            world,
            ball: Vec::with_capacity(1),
            players: Vec::with_capacity(2),
        }
    }

    pub fn init(&mut self) {
        self.init_ball();

        self.init_players();

        self.init_walls();
    }

    pub fn update(&mut self) {
        self.world.step();
    }

    pub fn handle_key_press(&mut self, key: &Key) {
        match key {
            &Key::W => {
                self.players[0].move_up(&mut self.world)
            },
            &Key::S => {
                self.players[0].move_down(&mut self.world)
            },
            &Key::Up => {
                self.players[1].move_up(&mut self.world)
            },
            &Key::Down => {
                self.players[1].move_down(&mut self.world)
            },
            _ => {},
        }
    }

    pub fn render<G>(&self, context: Context, graphics: &mut G)
    where
        G: Graphics,
    {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.ball[0].render(context, graphics, &self.world);

        for player in &self.players {
            player.render(context, graphics, &self.world);
        }

        self.render_walls(context, graphics)
    }

    fn init_ball(&mut self) {
        let ball_shape = ShapeHandle::new(Ball::new(ball::BALL_SIZE));
        let ball_collider = ColliderDesc::new(ball_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(2.0_f32, 0.0_f32)));

        let mut rb_desc = RigidBodyDesc::new()
            .collider(&ball_collider)
            .position(Isometry2::translation(50.0_f32, 50.0_f32))
            .velocity(Velocity2::linear(
                ball::BALL_HORIZONTAL_SPEED,
                ball::BALL_VERTICAL_SPEED,
            ));

        let rigid_body = rb_desc.build(&mut self.world);
        let ball_handle = rigid_body.handle();

        let ball = ball::PongBall::new(ball_handle);
        self.ball.push(ball);
    }

    fn init_players(&mut self) {
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::new(7.5_f32, 25.0_f32)));
        let player_collider = ColliderDesc::new(player_shape);
        let player_one_rb_desc = RigidBodyDesc::new().collider(&player_collider);

        let player_two_rb_desc = RigidBodyDesc::new().collider(&player_collider);

        let player_one_rigid_body = player_one_rb_desc
            .position(Isometry2::translation(50.0_f32, 200.0_f32))
            .status(BodyStatus::Kinematic)
            .build(&mut self.world);
        let player_one_handle = player_one_rigid_body.handle();

        let player_two_rigid_body = player_two_rb_desc
            .position(Isometry2::translation(735.0_f32, 300.0_f32))
            .status(BodyStatus::Kinematic)
            .build(&mut self.world);
        let player_two_handle = player_two_rigid_body.handle();

        let player_one = player::PongPlayer::new(player_one_handle);
        let player_two = player::PongPlayer::new(player_two_handle);

        self.players.push(player_one);
        self.players.push(player_two);
    }

    fn init_walls(&mut self) {
        let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            WALL_BODY_LENGTH,
            WALL_BODY_HEIGHT,
        )));
        let wall_collider = ColliderDesc::new(wall_shape);

        let mut rb_desc = RigidBodyDesc::new()
            .position(Isometry2::translation(
                TOP_WALL_X_POSITION,
                TOP_WALL_Y_POSITION,
            ))
            .status(BodyStatus::Static)
            .collider(&wall_collider);

        rb_desc.build(&mut self.world);

        rb_desc
            .position(Isometry2::translation(
                BOTTOM_WALL_X_POSITION,
                BOTTOM_WALL_Y_POSITION,
            ))
            .build(&mut self.world);
    }

    fn render_walls<G>(&self, context: Context, graphics: &mut G)
    where
        G: Graphics,
    {
        let empty_transform = context.transform.trans(0.0, 0.0);
        let rectangle = Rectangle::new(BLACK);
        rectangle.draw(
            [
                TOP_WALL_X_POSITION,
                TOP_WALL_Y_POSITION,
                WALL_DRAW_LENGTH,
                WALL_DRAW_HEIGHT,
            ],
            &context.draw_state,
            empty_transform,
            graphics,
        );
        rectangle.draw(
            [
                BOTTOM_WALL_X_POSITION,
                BOTTOM_WALL_Y_POSITION,
                WALL_DRAW_LENGTH,
                WALL_DRAW_HEIGHT,
            ],
            &context.draw_state,
            empty_transform,
            graphics,
        );
    }
}
