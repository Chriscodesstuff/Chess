use std::cmp::Ordering;

#[derive(PartialEq, Eq, Debug)]
pub enum Event {
    Update(String),
    Evaluate(i32),
    End,
    Test(i32),
}

#[derive(Eq, Debug)]
pub struct EventWrapper {
    pub time: u32,
    pub event: Event
}

impl Ord for EventWrapper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.time.cmp(&other.time).reverse()
    }
}

impl PartialOrd for EventWrapper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for EventWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.time == other.time
    }
}

impl EventWrapper {
    pub fn new(time: u32, event: Event) -> Self {
        Self {time, event}
    }
}