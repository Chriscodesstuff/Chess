use std::{
    collections::BinaryHeap,
    vec::Vec,
};

use crate::event::*;

pub struct EventQueue {
    time: u32,
    active: Vec<Event>,
    inactive: Vec<Event>,
    future: BinaryHeap<EventWrapper>
}

impl EventQueue {
    pub fn has_active_events(&self) -> bool {
        !self.active.is_empty()
    }

    pub fn next_active(&mut self) -> Option<Event> {
        self.active.pop()
    }

    pub fn has_inactive_events(&self) -> bool {
        !self.inactive.is_empty()
    }

    pub fn activate_inactive_events(&mut self) {
        self.active.append(&mut self.inactive);
    }

    pub fn advance_time(&mut self) {
        if let Some(event) = self.future.peek() {
            self.time = event.time;
            while !self.future.is_empty() {
                if self.future.peek().unwrap().time > self.time { break; }
                self.active.push(self.future.pop().unwrap().event)
            }
        }
    }

    pub fn schedule(&mut self, time: u32, event: Event) {
        if time <= self.time { self.inactive.push(event) }
        else { self.future.push(EventWrapper::new(time, event)); }
    }

    pub fn new() -> Self {
        EventQueue {
            time: 0, 
            active: Vec::new(), 
            inactive: Vec::new(), 
            future: BinaryHeap::new() 
        }
    }
}


/*
 * Tests for event_queue
 */
#[cfg(test)]
mod tests {
    use super::*;

    /*
     * test helpers
     */
    fn vector_has_event_testid(collection: &Vec<Event>, id:i32) -> bool {
        collection.iter().any(|event| match event {
            Event::Test(tid) => *tid == id,
            _ => false
        })
    }

    fn heap_has_event_wrapper_testid(collection: &BinaryHeap<EventWrapper>, id:i32) -> bool {
        collection.iter().any(|event_wrapper| match event_wrapper.event {
            Event::Test(tid) => tid == id,
            _ => false
        })
    }

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
    fn has_active_events_test_some() {
        let mut queue: EventQueue = EventQueue::new();

        queue.active.push(Event::Test(1));

        assert!(queue.has_active_events());
    }

    #[test]
    fn has_active_events_test_none() {
        let queue: EventQueue = EventQueue::new();

        assert!(!queue.has_active_events());
    }

    #[test]
    fn advance_time_test_progress() {
        let mut queue: EventQueue = EventQueue::new();
        queue.future.push(EventWrapper::new(1, Event::Test(1)));
        queue.future.push(EventWrapper::new(1, Event::Test(2)));
        queue.future.push(EventWrapper::new(2, Event::Test(3)));

        queue.advance_time();

        assert_eq!(queue.active.len(), 2, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 1, "<-- future contains 1 event");

        assert!(vector_has_event_testid(&queue.active, 1), "active contins event 1");
        assert!(vector_has_event_testid(&queue.active, 2), "active contins event 2");
        assert!(heap_has_event_wrapper_testid(&queue.future, 3), "future contins event 3");

        assert_eq!(queue.time, 1);
    }

    #[test]
    fn advance_time_test_end() {
        let mut queue: EventQueue = EventQueue::new();

        queue.advance_time();

        assert_eq!(queue.active.len(), 0, "<-- active contains 0 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 0, "<-- future contains 0 event");

        assert_eq!(queue.time, 0)
    }

    #[test]
    fn has_inactive_events_test_some() {
        let mut queue: EventQueue = EventQueue::new();

        queue.inactive.push(Event::Test(1));
        
        assert!(queue.has_inactive_events());
    }

    #[test]
    fn has_inactive_events_test_none() {
        let queue: EventQueue = EventQueue::new();

        assert!(!queue.has_inactive_events());
    }

    #[test]
    fn activate_inactive_events_test() {
        let mut queue: EventQueue = EventQueue::new();
        
        queue.inactive.push(Event::Test(1));
        queue.inactive.push(Event::Test(2));
        queue.future.push(EventWrapper::new(2, Event::Test(3)));

        queue.activate_inactive_events();

        assert_eq!(queue.active.len(), 2, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 1, "<-- future contains 1 event");

        assert!(vector_has_event_testid(&queue.active, 1), "active contins event 1");
        assert!(vector_has_event_testid(&queue.active, 2), "active contins event 2");
        assert!(heap_has_event_wrapper_testid(&queue.future, 3), "future contins event 3");
    }
    
    #[test]
    fn activate_inactive_events_test_empty_inactive() {
        let mut queue: EventQueue = EventQueue::new();
        
        queue.future.push(EventWrapper::new(1, Event::Test(1)));

        queue.activate_inactive_events();

        assert_eq!(queue.active.len(), 0, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 1, "<-- future contains 1 event");

        assert!(heap_has_event_wrapper_testid(&queue.future, 1), "future contins event 1");
    }
    
    #[test]
    fn activate_inactive_events_existing_active() {
        let mut queue: EventQueue = EventQueue::new();

        queue.active.push(Event::Test(1));
        queue.inactive.push(Event::Test(2));
        queue.inactive.push(Event::Test(3));
        queue.activate_inactive_events();

        assert_eq!(queue.active.len(), 3, "<-- active contains 3 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 0, "<-- future contains 0 event");

        assert!(vector_has_event_testid(&queue.active, 1), "active contins event 1");
        assert!(vector_has_event_testid(&queue.active, 2), "active contins event 2");
        assert!(vector_has_event_testid(&queue.active, 3), "active contins event 3");
    }

    #[test]
    fn next_active_test() {
        let mut queue: EventQueue = EventQueue::new();
        
        queue.active.push(Event::Test(1));
        queue.active.push(Event::Test(2));
        queue.active.push(Event::Test(3));

        let event = queue.next_active().unwrap();

        assert!(event_has_testid_in(&event, vec![1, 2, 3]));
    }
    
    #[test]
    fn next_active_test_no_active() {
        let mut queue: EventQueue = EventQueue::new();

        assert_eq!(queue.next_active(), None);
    }

    #[test]
    fn schedule_now() {
        let mut queue: EventQueue = EventQueue::new();
        queue.time = 1;

        queue.schedule(1, Event::Test(1));

        assert!(vector_has_event_testid(&queue.inactive, 1));
        assert_eq!(queue.time, 1);
    }

    #[test]
    fn schedule_later() {
        let mut queue: EventQueue = EventQueue::new();
        queue.time = 0;

        queue.schedule(1, Event::Test(1));

        assert!(heap_has_event_wrapper_testid(&queue.future, 1));
        assert_eq!(queue.time, 0);
    }
}