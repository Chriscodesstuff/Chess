mod event;
mod event_queue;


use event_queue::EventQueue;
use event::Event;

fn main() {
    let mut queue: EventQueue = Default::default();

    //main simulation loop
    loop {
        match queue.next_event() {
            Event::End => break,
            _ => ()
        }
    }
}