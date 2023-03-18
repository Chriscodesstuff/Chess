use std::{
    cmp::{Ordering},
    collections::BinaryHeap,
    vec::Vec,
};

#[derive(PartialEq, Eq, Debug)]
pub enum EventBody {
    Update(String),
    Evaluate(i32),
    End,
    Test(i32),
}

#[derive(Eq, Debug)]
pub struct Event {
    time: u32,
    pub body: EventBody
}

impl Ord for Event {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl Event {
    pub fn new(time: u32, body: EventBody) -> Self {
        Self {time, body}
    }
}

#[derive(Default)]
pub struct EventQueue {
    active: Vec<Event>,
    inactive: Vec<Event>,
    future: BinaryHeap<Event>
}

impl EventQueue {
    pub fn next_event(&mut self) -> Event{
        if self.active.is_empty() {
            self.activate_inactive_events();
        }
        return self.active.pop().unwrap()
    }

    fn activate_inactive_events(&mut self) {
        if self.inactive.is_empty() {
            self.advance_time();
        } else {
            self.active.append(&mut self.inactive);
        }
    }

    fn advance_time(&mut self) {
        if let Some(event) = self.future.peek() {
            let time = event.time;
            while !self.future.is_empty() {
                if self.future.peek().unwrap().time > time { break; }
                self.active.push(self.future.pop().unwrap())
            }
        } else {
            self.active.push(Event {
                time: 0,
                body: EventBody::End
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn event_has_testid(event: &Event, id: i32) -> bool {
        match event.body {
            EventBody::Test(tid) => tid == id,
            _ => false
        }
    }

    fn vector_has_event_testid(collection: &Vec<Event>, id:i32) -> bool {
        collection.iter().any(|event| match event.body {
            EventBody::Test(tid) => tid == id,
            _ => false
        })
    }

    fn heap_has_event_testid(collection: &BinaryHeap<Event>, id:i32) -> bool {
        collection.iter().any(|event| match event.body {
            EventBody::Test(tid) => tid == id,
            _ => false
        })
    }

    fn vector_has_event_end(collection: &Vec<Event>) -> bool {
        collection.iter().any(|event| match event.body {
            EventBody::End => true,
            _ => false
        })
    }

    fn event_is_end(event: &Event) -> bool {
        match event.body {
            EventBody::End => true,
            _ => false
        }
    }

    fn event_has_testid_in(event: &Event, ids: Vec<i32>) -> bool {
        match event.body {
            EventBody::Test(id) => ids.contains(&id),
            _ => false
        }
    }

    #[test]
    fn advance_time_test_progress() {
        let mut queue: EventQueue = Default::default();
        queue.future.push(Event::new(1, EventBody::Test(1)));
        queue.future.push(Event::new(1, EventBody::Test(2)));
        queue.future.push(Event::new(2, EventBody::Test(3)));

        queue.advance_time();

        assert_eq!(queue.active.len(), 2, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 1, "<-- future contains 1 event");

        assert!(vector_has_event_testid(&queue.active, 1), "active contins event 1");
        assert!(vector_has_event_testid(&queue.active, 2), "active contins event 2");
        assert!(heap_has_event_testid(&queue.future, 3), "future contins event 3");
    }

    #[test]
    fn advance_time_test_end() {
        let mut queue: EventQueue = Default::default();

        queue.advance_time();

        assert!(vector_has_event_end(&queue.active));
    }

    #[test]
    fn activate_inactive_events_test() {
        let mut queue: EventQueue = Default::default();
        
        queue.inactive.push(Event::new(1, EventBody::Test(1)));
        queue.inactive.push(Event::new(1, EventBody::Test(2)));
        queue.future.push(Event::new(2, EventBody::Test(3)));

        queue.activate_inactive_events();

        assert_eq!(queue.active.len(), 2, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 1, "<-- future contains 1 event");

        assert!(vector_has_event_testid(&queue.active, 1), "active contins event 1");
        assert!(vector_has_event_testid(&queue.active, 2), "active contins event 2");
        assert!(heap_has_event_testid(&queue.future, 3), "future contins event 3");
    }
    
    #[test]
    fn activate_inactive_events_test_advance_time() {
        let mut queue: EventQueue = Default::default();
        
        queue.future.push(Event::new(1, EventBody::Test(1)));
        queue.future.push(Event::new(1, EventBody::Test(2)));
        queue.future.push(Event::new(2, EventBody::Test(3)));

        queue.activate_inactive_events();

        assert_eq!(queue.active.len(), 2, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 1, "<-- future contains 1 event");

        assert!(vector_has_event_testid(&queue.active, 1), "active contins event 1");
        assert!(vector_has_event_testid(&queue.active, 2), "active contins event 2");
        assert!(heap_has_event_testid(&queue.future, 3), "future contins event 3");
    }
    
    #[test]
    fn activate_inactive_events_test_advance_end() {
        let mut queue: EventQueue = Default::default();

        queue.activate_inactive_events();

        assert_eq!(queue.active.len(), 1, "<-- active contains 2 events");
        assert_eq!(queue.inactive.len(), 0, "<-- inactive contains 0 events");
        assert_eq!(queue.future.len(), 0, "<-- future contains 1 event");

        assert!(vector_has_event_end(&queue.active));
    }

    #[test]
    fn next_event_test() {
        let mut queue: EventQueue = Default::default();
        
        queue.active.push(Event::new(1, EventBody::Test(1)));
        queue.active.push(Event::new(1, EventBody::Test(2)));
        queue.active.push(Event::new(1, EventBody::Test(3)));

        let event = queue.next_event();

        assert!(event_has_testid_in(&event, vec![1, 2, 3]));
    }
    
    #[test]
    fn next_event_test_activate_inactive() {
        let mut queue: EventQueue = Default::default();
        
        queue.inactive.push(Event::new(1, EventBody::Test(1)));
        queue.inactive.push(Event::new(1, EventBody::Test(2)));
        queue.inactive.push(Event::new(1, EventBody::Test(3)));

        let event = queue.next_event();

        assert!(event_has_testid_in(&event, vec![1, 2, 3]));
    }
    
    #[test]
    fn next_event_test_advance_time() {
        let mut queue: EventQueue = Default::default();

        let event = queue.next_event();

        assert!(event_is_end(&event));
    }
}