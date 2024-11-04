mod midi;
mod midi_input_handler;
mod time;

use macroquad::prelude::*;
use midi_input_handler::MidiInputHandler;

#[macroquad::main("MyGame")]
async fn main() {
    let mut midi_input = MidiInputHandler::new();
    loop {
        let mqtime = macroquad::time::get_time();

        let events = midi_input.process();

        if !events.is_empty() {
            println!("frame start {}", mqtime);
            println!("pressed a midi button");
        }

        if is_key_pressed(KeyCode::Space) {
            println!("pressed space");
            println!("frame start {}", mqtime);
        }

        next_frame().await
    }
}
