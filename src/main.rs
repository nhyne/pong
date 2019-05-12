extern crate piston_window;
extern crate find_folder;
extern crate nalgebra;
extern crate ncollide2d;
extern crate nphysics2d;

use piston_window::*;
use math::Matrix2d;

use nalgebra::{Point2, Vector2, Isometry2};
use ncollide2d::shape::{Ball, ShapeHandle};
use nphysics2d::object::{ColliderDesc, RigidBodyDesc, BodyHandle};
use nphysics2d::world::World;
use nphysics2d::algebra::Velocity2;
use core::borrow::Borrow;

const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 50.0;
const BALL_SIZE: f64 = 20.0;

struct Game {
    world: World<f64>,
    ball: Vec<PongBall>,
}

impl Game {
    fn new() -> Game {
        let mut world : World<f64> = World::new();
        Game {
            world,
            ball: Vec::with_capacity(1),
        }
    }

    fn init(&mut self) {
        let ball_shape = ShapeHandle::new(Ball::new(BALL_SIZE));
        let ball_collider = ColliderDesc::new(ball_shape);

        let mut rb_desc = RigidBodyDesc::new()
            .collider(&ball_collider)
            .velocity(Velocity2::linear(3.0, 2.0));

        let rigid_body = rb_desc.build(&mut self.world);
        let ball_handle = rigid_body.handle();

        let ball = PongBall::new(ball_handle);
        self.ball.push(ball);
    }

    fn update(&mut self) {
        self.world.step();
    }

    fn render<G> (&self, context: Context, graphics: &mut G)
        where G: Graphics {
        clear([0.8, 0.8, 0.8, 1.0], graphics);
        graphics.clear_stencil(0);

        self.ball[0].render(context, graphics, &self.world);
    }
}

struct Player {
    x_pos: f64,
    y_pos: f64,
    shape: Rectangle,
}

impl Player {
    fn render<G>(&self, draw_state: &DrawState, transform: Matrix2d, graphics: &mut G)
        where G: Graphics {
            self.shape.draw(
                [self.x_pos, self.y_pos, PLAYER_WIDTH, PLAYER_HEIGHT],
                draw_state,
                transform,
                graphics,
            )
        }

    fn new(x_pos: f64, y_pos: f64) -> Player {
        Player {
            x_pos,
            y_pos,
            shape: Rectangle::new(BLACK),
        }
    }

    fn move_up(&mut self) {
        self.y_pos += 5.0;
    }

    fn move_down(&mut self) {
        self.y_pos -= 5.0;
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

    //let mut player_1 = Player::new(50.0, 380.0);
    //let mut player_2 = Player::new(750.0, 380.0);


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
        // these things should be async, they're blocking
        //if let Some(Button::Keyboard(Key::A)) = e.press_args() {
        //    player_1.move_down()
        //}

        //if let Some(Button::Keyboard(Key::S)) = e.press_args() {
        //    player_1.move_up()
        //}

        //if let Some(Button::Keyboard(Key::K)) = e.press_args() {
        //    player_2.move_up()
        //}
        //if let Some(Button::Keyboard(Key::L)) = e.press_args() {
        //    player_2.move_down()
        //}

        //println!("{}", game.world.timestep());

        game.update();

        window.draw_2d(&e, |context, graphics| {
            game.render(context, graphics);
            //player_1.draw(&context.draw_state, context.transform, graphics);
            //player_2.draw(&context.draw_state, context.transform, graphics);
        });

    }
}
