extern crate piston_window;
extern crate find_folder;

use piston_window::draw_state::Blend;
use piston_window::*;

fn main() {
    println!("Press A to change blending");
    println!("Press S to change clip inside/out");

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
    let x_pos = 30.0;
    let mut y_pos = 0.0;

    window.set_lazy(true);
    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g| {
            clear([0.8, 0.8, 0.8, 1.0], g);
            g.clear_stencil(0);
            let cube_pos = c.transform.trans(x_pos, y_pos);
            let draw_state = c.draw_state.blend(blends[blend]);
            Rectangle::new([0.5, 1.0, 0.0, 0.3])
                .draw([0.0, 0.0, 100.0, 100.0], &draw_state, cube_pos, g);

        });

        if let Some(Button::Keyboard(Key::A)) = e.press_args() {
            y_pos = y_pos - 5.0;
        }

        if let Some(Button::Keyboard(Key::S)) = e.press_args() {
            y_pos = y_pos + 5.0;
        }
    }
}
