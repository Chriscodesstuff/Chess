mod event;
mod event_queue;
mod state;


use event_queue::EventQueue;
use event::Event;

fn main() {
    let mut queue: EventQueue = EventQueue::new();

    queue.schedule(1, Event::Test(1));

    //main simulation loop
    loop {
        if let Some(event) = timing_routine(&mut queue) {
            match event {
                _ => ()
            }
        } else {
            break;
        }
        
    }
}

fn timing_routine(queue: &mut EventQueue) -> Option<Event> {
    if !queue.has_active_events() {
        if queue.has_inactive_events() {
            queue.activate_inactive_events();
        } else {
            queue.advance_time();
            queue.activate_inactive_events();
        }
    }
    queue.next_active()
}

/*
 * Tests for event_queue
 */
#[cfg(test)]
mod tests {
    use super::*;
    use crate::event_queue::EventQueue;

    /*
     * test helpers
     */
    fn event_has_testid_in(event: &Event, ids: Vec<i32>) -> bool {
        match event {
            Event::Test(id) => ids.contains(&id),
            _ => false
        }
    }

    /*
     * tests
     */
    #[test]
    fn timing_routine_test() {
        let mut queue: EventQueue = EventQueue::new();

        queue.schedule(2, Event::Test(3));
        queue.schedule(1, Event::Test(1));
        queue.schedule(1, Event::Test(2));

        let event_1 = timing_routine(&mut queue).unwrap();
        let event_2 = timing_routine(&mut queue).unwrap();
        let event_3 = timing_routine(&mut queue).unwrap();

        assert!(event_has_testid_in(&event_1, vec![1, 2]));
        assert!(event_has_testid_in(&event_2, vec![1, 2]));
        assert!(event_has_testid_in(&event_3, vec![3]));
    }
}