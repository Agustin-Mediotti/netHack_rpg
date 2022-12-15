/*
    netHack_rpg by NetCreature

    compile & run: cargo run --release

    some notes:
    '--release' tag compiles the code with optimizations making it faster but worst on debugger.
    'tcod' is the bindings for 'libtcod'.


*/

use tcod::colors::*;
use tcod::console::*;

/* Actual size of window & max fps */

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

/*
    This struct will encapsulate all libtcod-related values
    This will help us pass them around to functions
*/
struct Tcod {
    root: Root,
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("netHack_rpg by NetCreature")
        .init();

    let mut tcod = Tcod { root };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    /* Main loop */

    while !tcod.root.window_closed() {
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root
            .put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        if exit {
            break;
        }
    }

    fn handle_keys(tcod: &mut Tcod, player_x: &mut i32, player_y: &mut i32) -> bool {
        use tcod::input::Key;
        use tcod::input::KeyCode::*;

        let key = tcod.root.wait_for_keypress(true);

        match key {
            Key {
                code: Enter,
                alt: true,
                ..
            } => {
                let fullscreen = tcod.root.is_fullscreen();
                tcod.root.set_fullscreen(!fullscreen);
            }
            Key { code: Escape, .. } => return true,
            Key { code: Up, .. } => *player_y -= 1,
            Key { code: Down, .. } => *player_y += 1,
            Key { code: Left, .. } => *player_x -= 1,
            Key { code: Right, .. } => *player_x += 1,

            _ => {} // If any other key is pressed do nothing
        }

        /* The two dots at the end mean "I don't care about the other fields" */

        false
    }
}
