# NetHacker RPG
Written in Rust by _NetCreature_

## What is it?
NetHacker is a RogueLike RPG game made with lots of ASCII Art & nerd humor

## Build, Compile, Run
* `cargo build --release`
* `cargo run --release`

## Notes:
* `--release` tag compiles the code with optimizations making it faster but worst on debugger.
* _'tcod'_ is the bindings for _'libtcod'_.
* An alternative to create a map is to treat walls and everything else in the map as just another `Object` and store them there. This would make the game structure simplier (everything is an `Object`) and more flexible (adding HP to make it destructible or damage if it suppose to.)