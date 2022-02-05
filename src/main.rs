extern crate midi_control;
extern crate midir;
extern crate winput;

use std::collections::HashMap;
use std::sync::mpsc::channel;
use std::thread;
use std::time;

use midi_control::MidiMessage;
use winput::{Action, Input, Vk};

fn main() {

    /* Start Configuration */

    // The name of your MIDI device.
    let midi_device_name: &str = "TSMIDI2.0";

    // The configuration for the CC messages.
    let configuration = HashMap::from([
        (
            53_u8, // ctrl+c
            vec![
                Input::from_vk(Vk::Control, Action::Press),
                Input::from_vk(Vk::C, Action::Press),
                Input::from_vk(Vk::Control, Action::Release),
                Input::from_vk(Vk::C, Action::Release),
            ],
        ),
        (
            52_u8, // ctrl+v
            vec![
                Input::from_vk(Vk::Control, Action::Press),
                Input::from_vk(Vk::V, Action::Press),
                Input::from_vk(Vk::Control, Action::Release),
                Input::from_vk(Vk::V, Action::Release),
            ],
        ),
    ]);

    /* End Configuration */

    // create midi input.
    let midi_input = midir::MidiInput::new("MIDITest").unwrap();

    // find our device.
    let device_port = find_port(&midi_input, midi_device_name);
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
        midi_device_name,
        move |_, data, sender| {
            let msg = MidiMessage::from(data);
            if let Err(e) = sender.send(msg) {
                println!("Failed to send message to channel: {:?}", e);
            }
        },
        sender,
    );

    // set a loop to handle received messages.
    let hundred_millis = time::Duration::from_millis(100);
    loop {
        if let Ok(msg) = receiver.recv() {
            match msg {
                MidiMessage::ControlChange(_, control_event) => {
                    execute_keystrokes(&configuration, &control_event.control)
                } // handle accordingly.
                _ => println!("Unsupported MIDI message: {:?}", msg),
            };
        }
        thread::sleep(hundred_millis);
    }
}

// find_port will find the correct port associated to the midi device.
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

// execute keystrokes searches for the corresponding CC in the configuration
// and if it exists, executes the inputs.
fn execute_keystrokes(configuration: &HashMap<u8, std::vec::Vec<winput::Input>>, cc: &u8) {
    println!("CC {:?} received", cc);

    if let Some(inputs) = configuration.get(cc) {
        println!("Executing keystrokes for CC {:?}", cc);
        winput::send_inputs(&inputs);
    };
}
