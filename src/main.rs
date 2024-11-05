mod midi;
mod midi_input_handler;
mod time;

use macroquad::prelude::*;
use midi_input_handler::{Events, MidiInputHandler};
use time::current_time_millis;

fn mq_get_time_ms() -> u128 {
    (macroquad::time::get_time() * 1000.) as u128
}

// TODO: Draw a picture
// we want to compare a the timestamp of a game frame
// real - midi = actual_midi

#[macroquad::main("MyGame")]
async fn main() {
    let real_start_time = current_time_millis();
    let mq_start_time_ms = mq_get_time_ms();
    // TODO: we also need Kira clock time :welp:
    println!("real_start_time: {real_start_time}");
    println!("mq_start_time_ms: {mq_start_time_ms}");

    let mut max_diff = 0;
    let mut midi_input = MidiInputHandler::new();
    loop {
        let real_time = current_time_millis();
        let mqtime: u128 = mq_get_time_ms();
        // get_frame_time() // potentially useful to see if frame times are uneven

        let events = midi_input.process();

        if !events.is_empty() {
            println!("pressed a midi button");
            println!("\tframe start (ms) .. mqtime     = {}", mqtime);
            println!("\tframe start (ms) .. realtime   = {}", real_time);
            for e in events {
                match e {
                    Events::Hit(h) => {
                        println!("hit .. realtime: {:?}", h.raw_data.non_midi_timestamp_ms);
                        let diff_midi_hit_to_frame_time: i128 =
                            real_time as i128 - h.raw_data.non_midi_timestamp_ms as i128;
                        println!("diff = {}", diff_midi_hit_to_frame_time);
                        if diff_midi_hit_to_frame_time > max_diff {
                            max_diff = diff_midi_hit_to_frame_time;
                        }
                        println!("max_diff = {}", max_diff);
                    }
                }
                // TODO: Can I surface the exact midi input timing (maybe just via modifying my local)
            }
        }

        if is_key_pressed(KeyCode::Space) {
            println!("frame start (ms) {}", mqtime);
            println!("pressed space");
            // TODO: Can I surface the exact keyboard input timing (hack / vendor macroquad lib)
        }

        next_frame().await
    }
}
