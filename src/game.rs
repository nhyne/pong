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
use nphysics2d::object::{BodyPartHandle, BodyStatus, ColliderDesc, RigidBodyDesc};
use nphysics2d::world::World;

use std::collections::HashSet;

const BLACK: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

const WALL_DRAW_LENGTH: f64 = 800.0;
const WALL_DRAW_HEIGHT: f64 = 20.0;
const WALL_BODY_LENGTH: f64 = 800.0;
const WALL_BODY_HEIGHT: f64 = 0.5;

const TOP_WALL_X_POSITION: f64 = 0.0;
const TOP_WALL_Y_POSITION: f64 = 0.0;
const BOTTOM_WALL_X_POSITION: f64 = 0.0;
const BOTTOM_WALL_Y_POSITION: f64 = 400.0;

pub struct Game {
    world: World<f64>,
    ball: ball::PongBall,
    player_one: player::PongPlayer,
    player_two: player::PongPlayer,
    player_one_score: u8,
    player_two_score: u8,
    keys_pressed: HashSet<piston_window::Key>,
}

impl Game {
    pub fn new() -> Game {
        let mut world: World<f64> = World::new();

        let ball = Game::init_ball(&mut world);
        let player_one = Game::init_player(&mut world, (50.0, 200.0), Key::W, Key::S);
        let player_two = Game::init_player(&mut world, (735.0, 300.0), Key::Up, Key::Down);
        Game::init_walls(&mut world);

        Game {
            world,
            ball,
            player_one,
            player_two,
            keys_pressed: HashSet::new(),
            player_one_score: 0,
            player_two_score: 0,
        }
    }

    pub fn update(&mut self) {
        self.player_one.update(&mut self.world, &self.keys_pressed);
        self.player_two.update(&mut self.world, &self.keys_pressed);
        self.check_goal();

        self.world.step();
    }

    pub fn handle_keyboard_event(&mut self, key: ButtonArgs) {
        match key.state {
            ButtonState::Press => {
                if let Button::Keyboard(key) = key.button {
                    self.keys_pressed.insert(key);
                }
            }
            ButtonState::Release => {
                if let Button::Keyboard(key) = key.button {
                    self.keys_pressed.remove(&key);
                }
            }
        }
    }

    pub fn render<G: Graphics>(&self, context: Context, graphics: &mut G) {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.ball.render(context, graphics, &self.world);

        self.player_one.render(context, graphics, &self.world);
        self.player_two.render(context, graphics, &self.world);

        self.render_walls(context, graphics)
    }

    fn check_goal(&mut self) {
        if let Some(ball_pos) = self.ball.get_position(&self.world) {
            if ball_pos[0] > 800.0 {
                self.player_one_score += 1;
                self.reset_ball();
            } else if ball_pos[0] < 0.0 {
                self.player_two_score += 1;
                self.reset_ball();
            }
        }
    }

    fn reset_ball(&mut self) {
        self.world.remove_bodies(&[self.ball.body]);
        self.ball = Game::init_ball(&mut self.world);
    }

    // TODO It seems like the world knows a little too much about the ball
    //   and the material it is constructed out of
    fn init_ball(world: &mut World<f64>) -> ball::PongBall {
        let ball_shape = ShapeHandle::new(Ball::new(ball::BALL_SIZE));
        let ball_collider = ColliderDesc::new(ball_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(2.0, 0.0)));

        let mut rb_desc = RigidBodyDesc::new()
            .collider(&ball_collider)
            .position(Isometry2::translation(50.0, 50.0))
            .velocity(Velocity2::linear(
                ball::BALL_HORIZONTAL_SPEED,
                ball::BALL_VERTICAL_SPEED,
            ));

        let rigid_body = rb_desc.build(world);
        let ball_handle = rigid_body.handle();

        ball::PongBall::new(ball_handle)
    }

    fn init_player(
        world: &mut World<f64>,
        position: (f64, f64),
        up_key: Key,
        down_key: Key,
    ) -> player::PongPlayer {
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::new(7.5, 25.0)));
        let player_collider = ColliderDesc::new(player_shape)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)));
        let player_rb_desc = RigidBodyDesc::new().collider(&player_collider);

        let player_rigid_body = player_rb_desc
            .position(Isometry2::translation(position.0, position.1))
            .status(BodyStatus::Kinematic)
            .build(world);
        let player_handle = player_rigid_body.handle();

        player::PongPlayer::new(player_handle, up_key, down_key)
    }

    fn init_walls(world: &mut World<f64>) {
        let wall_shape = ShapeHandle::new(Cuboid::new(Vector2::new(
            WALL_BODY_LENGTH,
            WALL_BODY_HEIGHT,
        )));

        let wall_shape2 = ShapeHandle::new(Cuboid::new(Vector2::new(
            WALL_BODY_LENGTH,
            WALL_BODY_HEIGHT,
        )));

        ColliderDesc::new(wall_shape)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)))
            .position(Isometry2::translation(
                TOP_WALL_X_POSITION,
                TOP_WALL_Y_POSITION,
            ))
            .build_with_parent(BodyPartHandle::ground(), world);

        ColliderDesc::new(wall_shape2)
            .material(MaterialHandle::new(BasicMaterial::new(0.0, 0.0)))
            .position(Isometry2::translation(
                BOTTOM_WALL_X_POSITION,
                BOTTOM_WALL_Y_POSITION,
            ))
            .build_with_parent(BodyPartHandle::ground(), world);
    }

    fn render_walls<G: Graphics>(&self, context: Context, graphics: &mut G) {
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
