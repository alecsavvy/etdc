use crate::state::CliInput;
use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

pub fn cli_input(
    mut char_evr: EventReader<ReceivedCharacter>,
    keys: Res<Input<KeyCode>>,
    mut string: Local<String>,
) {
    for ev in char_evr.iter() {
        println!("Got char: '{}'", ev.char);
        string.push(ev.char);
    }

    if keys.just_pressed(KeyCode::Return) {
        println!("Text input: {}", *string);
        string.clear();
    }
}
