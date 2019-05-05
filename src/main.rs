extern crate piston_window;
extern crate find_folder;

use piston_window::draw_state::Blend;
use piston_window::*;

use math::Matrix2d;

const BLACK: [f32;4] = [0.0, 0.0, 0.0, 1.0];
const PLAYER_WIDTH: f64 = 15.0;
const PLAYER_HEIGHT: f64 = 50.0;

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

fn main() {

    let mut player_1 = Player::new(0.0, 0.0);

    let mut window: PistonWindow = WindowSettings::new(
        "piston: draw_state",
        [800, 800]
    )
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    let blends = [Blend::Alpha, Blend::Add, Blend::Invert, Blend::Multiply];
    let blend = 0;

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |context, graphics| {
            clear([0.8, 0.8, 0.8, 1.0], graphics);
            graphics.clear_stencil(0);
            let empty_transform = context.transform.trans(0.0, 0.0);
            let draw_state = context.draw_state.blend(blends[blend]);
            player_1.draw(&draw_state, empty_transform, graphics);

        });


        // these things should be async, they're blocking
        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            player_1.move_down()
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
           player_1.move_up()
        }
    }
}
