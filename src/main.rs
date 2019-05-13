mod game;

extern crate find_folder;
extern crate piston_window;

use piston_window::*;

fn main() {
    let mut game = game::Game::new();
    game.init();

    let mut window: PistonWindow = WindowSettings::new("piston: draw_state", [800, 420])
        .exit_on_esc(true)
        .samples(4)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        // TODO Make these functions async because they're blocking
        if let Some(Button::Keyboard(key)) = e.press_args() {
            game.handle_key_press(&key);
        }

        game.update();

        window.draw_2d(&e, |context, graphics| {
            game.render(context, graphics);
        });
    }
}
