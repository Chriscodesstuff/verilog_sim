

use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::mem::replace;

#[derive(Debug, PartialEq)]
enum Event {
    Update,
    Other,
    None
}

struct EventQueue {
    time: u32,
    active: VecDeque<Event>, 
    inactive: VecDeque<Event>,
    future: BTreeMap<u32, VecDeque<Event>>
}

impl EventQueue {
    fn new() -> EventQueue {
        EventQueue {
            time: 0,
            active: VecDeque::new(),
            inactive: VecDeque::new(),
            future: BTreeMap::new()
        }
    }
    fn schedule_event(&mut self, e: Event, t: u32) {
        if self.time == t {
            self.inactive.push_front(e);
            return;
        }
        if self.future.contains_key(&t) {
            self.future.get_mut(&t).unwrap().push_front(e);
            return;
        }
        self.future.insert(t, {
            let mut q = VecDeque::new();
            q.push_front(e);
            q
        }); 
    }

    fn activate_inactive(&mut self) {
        let q = std::mem::replace(&mut self.inactive, VecDeque::new());
        self.inactive = q; 
    }

    fn activate_future(&mut self) {
        if let Some((t, _)) = self.future.iter().next() {
            self.time = *t;
        } else { return; }
        println!("map: {:?}", self.future);
        let mut r = self.future.remove(&self.time).unwrap();
        let q = std::mem::replace(&mut r, VecDeque::new());
        self.active = q; 
    }

    fn pop_event(&mut self) -> Option<(Event, u32)> {
        if self.active.is_empty() {
            if self.inactive.is_empty() {
                self.activate_future();
            } else if self.future.is_empty() {
                return None;
            }
            self.activate_inactive();
        } 
        if let Some(e) = self.active.pop_back() {
            return Some((e, self.time));
        }   
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_event_queue() {
        let mut eq = EventQueue::new();
        eq.schedule_event(Event::Update, 20);
        eq.schedule_event(Event::Other, 10);
        eq.schedule_event(Event::None, 30);
        eq.schedule_event(Event::None, 10);
        assert_eq!(eq.pop_event(), Some((Event::Other, 10)));
        assert_eq!(eq.pop_event(), Some((Event::None, 10)));
        assert_eq!(eq.pop_event(), Some((Event::Update, 20)));
        assert_eq!(eq.pop_event(), Some((Event::None, 30)));
    }

    #[test]
    fn test_constructor() {
        let mut eq = EventQueue::new(); 
        assert_eq!(eq.pop_event(), None);
    }

}