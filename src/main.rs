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

    /* Main loop */

    while !tcod.root.window_closed() {
        tcod.root.set_default_foreground(WHITE);
        tcod.root.clear();
        tcod.root.put_char(1, 1, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
    }
}
