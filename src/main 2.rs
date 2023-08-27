extern crate midir;

use midir::{MidiOutput, MidiOutputConnection};
use std::io::{stdout, stdin, Write};
use std::time::Duration;
use std::thread::sleep;

fn main() {
    // Initialize MIDI output
    let midi_out = MidiOutput::new("MyMidiOutput").expect("Failed to create MIDI output");
    let out_ports = midi_out.ports();

    //let out_port: &MidiOutputPort = &out_ports[1];
    let out_port = match out_ports.len() {
        0 => Err("no output port found".into()),
        1 => {
            println!(
                "Choosing the only available output port: {}",
                midi_out.port_name(&out_ports[0]).unwrap()
            );
            Ok(&out_ports[0])
        }
        _ => {
            println!("\nAvailable output ports:");
            for (i, p) in out_ports.iter().enumerate() {
                println!("{}: {}", i, midi_out.port_name(p).unwrap());
            }
            print!("Please select output port: ");
            stdout().flush().unwrap();
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();
            out_ports
                .get(input.trim().parse::<usize>().unwrap())
                .ok_or("invalid output port selected")
        }
    };
    
    let mut midi_conn = midi_out.connect(out_port.unwrap(), "midir-test").expect("Failed to connect to MIDI output");

    // Define the MIDI chord notes
    let chord_notes = vec![60, 64, 67]; // C Major chord

    // Send MIDI messages to play the chord
    play_chord(&mut midi_conn, &chord_notes);

    // Keep the program running for a while to let the notes play
    sleep(Duration::from_secs(2));
}

fn play_chord(midi_conn: &mut MidiOutputConnection, notes: &[u8]) {
    for &note in notes {
        // Send Note On message
        let message = &[0x90, note, 64]; // Note On, note value, velocity
        midi_conn.send(message).expect("Failed to send MIDI message");

        // Delay for a short time (e.g., 100ms)
        sleep(Duration::from_millis(100));

        // Send Note Off message
        let message = &[0x80, note, 64]; // Note Off, note value, velocity
        midi_conn.send(message).expect("Failed to send MIDI message");
    }
}
