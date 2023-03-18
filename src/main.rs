mod event_queue;

use event_queue::*;

fn main() {
    let mut queue: EventQueue = Default::default();

    //main simulation loop
    loop {
        match queue.next_event().body {
            EventBody::End => break,
            _ => ()
        }
    }
}