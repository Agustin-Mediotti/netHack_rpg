/*
    Copyright 2022 NetCreature

    File Author: NetCreature
*/

use tcod::colors::*;
use tcod::console::*;

/* Actual size of window & max fps */

const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

/*
    This struct encapsulates all libtcod-related values
    and help us pass them around functions easily
*/
struct Tcod {
    root: Root,
    con: Offscreen,
}

fn main() {
    let root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("netHack_rpg by NetCreature")
        .init();
    let con = Offscreen::new(SCREEN_WIDTH, SCREEN_HEIGHT);

    let mut tcod = Tcod { root, con };

    tcod::system::set_fps(LIMIT_FPS);

    let mut player_x = SCREEN_WIDTH / 2;
    let mut player_y = SCREEN_HEIGHT / 2;

    /* Main loop */

    while !tcod.root.window_closed() {
        let exit = handle_keys(&mut tcod, &mut player_x, &mut player_y);
        tcod.con.set_default_foreground(WHITE);
        tcod.con.clear();
        tcod.con
            .put_char(player_x, player_y, '@', BackgroundFlag::None);
        tcod.root.flush();
        tcod.root.wait_for_keypress(true);
        blit(
            &tcod.con,
            (0, 0),
            (SCREEN_WIDTH, SCREEN_HEIGHT),
            &mut tcod.root,
            (0, 0),
            1.0,
            1.0,
        );
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
