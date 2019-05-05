extern crate piston_window;
extern crate find_folder;

use piston_window::*;
use math::Matrix2d;

const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 50.0;
const BALL_SIZE: f64 = 20.0;

struct Player {
    x_pos: f64,
    y_pos: f64,
    shape: Rectangle,
}

impl Player {
    fn draw<G>(&self, draw_state: &DrawState, transform: Matrix2d, graphics: &mut G)
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

struct Ball {
    x_pos: f64,
    y_pos: f64,
    x_velocity: f64,
    y_velocity: f64,
    shape: Rectangle,
}

impl Ball {
    fn new(x_pos: f64, y_pos: f64) -> Ball {
        Ball {
           x_pos,
            y_pos,
            shape: Rectangle::new(BLACK),
            x_velocity: 0.05,
            y_velocity: 0.0,
        }
    }

    fn update_pos(&mut self) {
        self.x_pos += self.x_velocity;
        self.y_pos += self.y_velocity;
    }

    fn draw<G> (&self, draw_state: &DrawState, transform: Matrix2d, graphics: &mut G)
        where G: Graphics {
        self.shape.draw(
            [self.x_pos, self.y_pos, BALL_SIZE, BALL_SIZE],
            draw_state,
            transform,
            graphics
        )
    }
}

fn main() {

    let mut player_1 = Player::new(50.0, 380.0);
    let mut player_2 = Player::new(750.0, 380.0);
    let mut ball = Ball::new(400.0, 400.0);

    let mut window: PistonWindow = WindowSettings::new(
        "piston: draw_state",
        [800, 800]
    )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        ball.update_pos();
        window.draw_2d(&e, |context, graphics| {
            clear([0.8, 0.8, 0.8, 1.0], graphics);
            graphics.clear_stencil(0);
            player_1.draw(&context.draw_state, context.transform, graphics);
            player_2.draw(&context.draw_state, context.transform, graphics);
            ball.draw(&context.draw_state, context.transform, graphics);
        });

        // these things should be async, they're blocking
        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            player_1.move_down()
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
           player_1.move_up()
        }

        if let Some(Button::Keyboard(Key::K)) = e.press_args() {
            player_2.move_up()
        }
        if let Some(Button::Keyboard(Key::L)) = e.press_args() {
            player_2.move_down()
        }

    }
}
