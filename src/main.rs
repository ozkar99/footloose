extern crate midi_control;
extern crate midir;

use std::sync::mpsc::channel;
use std::thread;
use std::time;

use midi_control::MidiMessage;

// The name of your MIDI device.
const MIDI_DEVICE_NAME: &str = "TSMIDI2.0";

fn main() {

    // create midi input.
    let midi_input = midir::MidiInput::new("MIDITest").unwrap();

    // find our device.
    let device_port = find_port(&midi_input, MIDI_DEVICE_NAME);
    if device_port.is_none() {
        println!("Input device not found!");
        return;
    }


    // create channel for sending/receiving from inside the loop.
    let (sender, receiver) = channel::<MidiMessage>();

    // set-up thread on any message received from the device,
    // just send to the channel.
    let device_port = device_port.unwrap();
    let _connect_in = midi_input.connect(
        &device_port,
        MIDI_DEVICE_NAME,
        move |_, data, sender| {
            let msg = MidiMessage::from(data);
            if let Err(e) = sender.send(msg) {
                println!("Failed to send message to channel: {:?}", e);
                return;
            }
        },
        sender,
    );


    // set a loop to handle received messages.
    let hundred_millis = time::Duration::from_millis(100);
    loop {
          if let Ok(msg) = receiver.recv() {
            match msg {
                MidiMessage::ControlChange(_, control_event) => println!("CC {:?} received", control_event.control), // handle accordingly.
                _ => println!("Unsupported MIDI message: {:?}", msg),
            };
          }
        thread::sleep(hundred_millis);
    }
}

fn find_port<T>(midi_io: &T, device_port_name: &str) -> Option<T::Port>
where
    T: midir::MidiIO,
{
    let mut device_port: Option<T::Port> = None;
    for port in midi_io.ports() {
        if let Ok(port_name) = midi_io.port_name(&port) {
            if port_name.contains(device_port_name) {
                device_port = Some(port);
                break;
            }
        }
    }
    device_port
}
