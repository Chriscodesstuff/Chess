mod event_queue;

use event_queue::*;

fn main() {
    let mut queue: EventQueue = Default::default();

    //main simulation loop
    loop {
        match next_event(&mut queue).body {
            EventBody::End => break,
            _ => ()
        }
    }
}