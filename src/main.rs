use midir::MidiInput;
use ghakuf::messages::{MidiEvent, MidiEventBuilder};

// Display name for MIDI client
const CLIENT_NAME:     &'static str = "@selenologist MIDI monitor";
const INPUT_PORT_NAME: &'static str = "@selenologist MIDI monitor input";

#[derive(Default)]
struct HandlerData {
    last_time: u64
}

fn handler(time: u64, midi_data: &[u8], data: &mut HandlerData) {
    // split time into whole seconds and the remaining microseconds
    let time_seconds = time / 1_000_000;
    let time_micros  = time % 1_000_000;

    let delta         = time - data.last_time;
    let delta_seconds = delta / 1_000_000;
    let delta_micros  = delta % 1_000_000;

    data.last_time = time;
    
    // parse the midi message
    let mut mev_builder = MidiEventBuilder::new(midi_data[0]);
    for byte in &midi_data[1..midi_data.len()] {
        mev_builder.push(*byte);
    }
    
    let event: MidiEvent = mev_builder.build();

    // print the time and message
    println!("[{}.{:06}(+{}.{:06})] {:?}",
            time_seconds, time_micros, delta_seconds, delta_micros, event);
}

fn main() {
    // Virtual Inputs aren't supported on Windows.
    // So this program can't be compiled for Windows.
    use midir::os::unix::VirtualInput;

    let input =
        MidiInput::new(CLIENT_NAME)
        .expect("Failed to create MIDI input");

    let port = input
        .create_virtual(INPUT_PORT_NAME, handler, HandlerData::default())
        .expect("Failed to create MIDI input port");

    // input events will now call the handler

    // block on a channel with no other writers in order to sleep forever
    // (Ctrl-C should still terminate the program as normal)
    let (tx, rx) = std::sync::mpsc::channel();
    rx.recv().unwrap();

    // this point is now unreachable. 'Use' tx so it won't be optimised out / dropped early.
    tx.send(()).unwrap();

    // close the port manually so that `port` is not 'unused' and therefore won't be optimised out
    port.close();
}
