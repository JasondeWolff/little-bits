use std::collections::VecDeque;

pub struct HandleQueue {
    handles: VecDeque<u64>
}

impl HandleQueue {
    pub fn new(capacity: u64) -> Self {
        let mut queue = HandleQueue {
            handles: VecDeque::with_capacity(capacity as usize)
        };

        for i in 0..capacity {
            queue.handles.push_back(i);
        }

        queue
    }

    pub fn create(&mut self) -> u64 {
        let handle = self.handles.pop_front().expect("Failed to create new handle. (No more handles availale)");
        handle
    }

    pub fn destroy(&mut self, handle: u64) {
        self.handles.push_back(handle);
    }
}