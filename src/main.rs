extern crate piston_window;
extern crate find_folder;
extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;

use piston_window::*;
use math::Matrix2d;

use nalgebra::{Point2, Vector2, Isometry2};
use ncollide2d::shape::{Ball, ShapeHandle, Cuboid};
use nphysics2d::object::{ColliderDesc, RigidBodyDesc, BodyHandle, BodyStatus};
use nphysics2d::world::World;
use nphysics2d::algebra::Velocity2;
use core::borrow::{Borrow, BorrowMut};
use nphysics2d::material::{MaterialHandle, BasicMaterial};

const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 50.0;

const BALL_SIZE: f64 = 20.0;
const BALL_HORIZONTAL_SPEED: f64 = 6.0;
const BALL_VERTICAL_SPEED: f64 = 4.0;
const BALL_SPEED: Velocity2<f64> = Velocity2::linear(BALL_HORIZONTAL_SPEED, BALL_VERTICAL_SPEED);

struct Game {
    world: World<f64>,
    ball: Vec<PongBall>,
    players: Vec<PongPlayer>,
}

impl Game {
    fn new() -> Game {
        let mut world : World<f64> = World::new();
        Game {
            world,
            ball: Vec::with_capacity(1),
            players: Vec::with_capacity(2),
        }
    }

    fn init(&mut self) {
        //TODO Write a ball init function specifically
        let ball_shape = ShapeHandle::new(Ball::new(BALL_SIZE));
        let ball_collider = ColliderDesc::new(ball_shape)
            .density(1.0)
            .material(MaterialHandle::new(BasicMaterial::new(2.0, 0.0)));

        let mut rb_desc = RigidBodyDesc::new()
            .collider(&ball_collider)
            .velocity(BALL_SPEED);

        let rigid_body = rb_desc.build(&mut self.world);
        let ball_handle = rigid_body.handle();

        let ball = PongBall::new(ball_handle);
        self.ball.push(ball);

        //TODO Write a player init function specifically
        let player_shape = ShapeHandle::new(Cuboid::new(Vector2::repeat(3.0)));
        let player_collider = ColliderDesc::new(player_shape);
        let mut player_one_rb_desc = RigidBodyDesc::new()
            .collider(&player_collider);

        let mut player_two_rb_desc = RigidBodyDesc::new()
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

        let player_one = PongPlayer::new(player_one_handle);
        let player_two = PongPlayer::new(player_two_handle);

        self.players.push(player_one);
        self.players.push(player_two);

    }

    fn update(&mut self) {
        self.world.step();
    }

    fn render<G> (&self, context: Context, graphics: &mut G)
        where G: Graphics {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.ball[0].render(context, graphics, &self.world);

        for player in &self.players {
            player.render(context, graphics, &self.world);
        }

    }
}

struct PongPlayer {
    shape: Rectangle,
    body: BodyHandle,
}

impl PongPlayer {
    fn render<G>(&self, context: Context, graphics: &mut G, world: &World<f64>)
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

    fn new(body: BodyHandle) -> PongPlayer {
        PongPlayer {
            body,
            shape: Rectangle::new(BLACK),
        }
    }

    fn move_up(&mut self, world: &mut World<f64>) {
        let player_body = world.rigid_body_mut(self.body);
        match player_body {
            None => {},
            Some(b) => {
                let player_body = b.borrow_mut();
                let current_pos = b.position().translation.vector;
                player_body.set_position(Isometry2::translation(current_pos[0], current_pos[1] + 5.0))
            }
        }
    }

    fn move_down(&mut self, world: &mut World<f64>) {
        let player_body = world.rigid_body_mut(self.body);
        match player_body {
            None => {},
            Some(b) => {
                let player_body = b.borrow_mut();
                let current_pos = b.position().translation.vector;
                player_body.set_position(Isometry2::translation(current_pos[0], current_pos[1] - 5.0))
            }
        }
    }
}

struct PongBall {
    shape: Rectangle,
    body: BodyHandle,
}

impl PongBall {
    fn new(body: BodyHandle) -> PongBall {
        PongBall {
            shape: Rectangle::new(BLACK),
            body,
        }
    }

    fn render<G> (&self, context: Context, graphics: &mut G, world: &World<f64>)
    where G: Graphics {
        let body = world.rigid_body(self.body);
        match body {
            None => {},
            Some(b) => {
                let ball_body = b.borrow();
                let pos = ball_body.position().translation.vector;
                self.shape.draw(
                    [pos[0], pos[1], BALL_SIZE, BALL_SIZE],
                    &context.draw_state,
                    context.transform,
                    graphics
                );
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.init();

    let mut window: PistonWindow = WindowSettings::new(
        "piston: draw_state",
        [800, 800]
    )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        // TODO Make these functions async because they're blocking
        //if let Some(Button::Keyboard(Key::A)) = e.press_args() {
        //    player_1.move_down(&mut game.world)
        //}

        //if let Some(Button::Keyboard(Key::S)) = e.press_args() {
        //    player_1.move_up(&mut game.world)
        //}

        //if let Some(Button::Keyboard(Key::K)) = e.press_args() {
        //    player_2.move_up()
        //}
        //if let Some(Button::Keyboard(Key::L)) = e.press_args() {
        //    player_2.move_down()
        //}

        game.update();

        window.draw_2d(&e, |context, graphics| {
            game.render(context, graphics);
        });

    }
}
