use std::{
    fmt::Debug,
    sync::{Mutex, Weak},
};

pub trait Handler<E>: Send + Debug {
    fn handle(&mut self, event: &E);
}

#[derive(Debug)]
pub struct Emitter<E> {
    pub handlers: Vec<Weak<Mutex<dyn Handler<E>>>>,
}

impl<E> Emitter<E> {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn subscribe<H>(&mut self, handler: Weak<Mutex<H>>)
    where
        H: Handler<E> + 'static,
    {
        self.handlers.push(handler);
    }

    pub fn emit(&mut self, event: E) {
        self.handlers.retain(|handler| match handler.upgrade() {
            Some(handler) => {
                // Handler still exists, call and keep in vector
                handler.lock().unwrap().handle(&event);
                true
            }
            None => {
                // Handler doesn't exists anymore, remove from vector
                false
            }
        });
    }
}

impl<E> Default for Emitter<E> {
    fn default() -> Self {
        Self::new()
    }
}

pub trait Subscribable {
    type Event;
    fn subscribe<H>(&mut self, handler: Weak<Mutex<H>>)
    where
        H: Handler<Self::Event> + 'static;
}

impl<E> Subscribable for Emitter<E> {
    type Event = E;

    fn subscribe<H>(&mut self, handler: Weak<Mutex<H>>)
    where
        H: Handler<E> + 'static,
    {
        self.subscribe(handler)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;

    #[derive(Debug)]
    struct Counter {
        value: i32,
    }

    impl Handler<i32> for Counter {
        fn handle(&mut self, event: &i32) {
            self.value += event;
        }
    }

    #[test]
    fn should_register_handler_and_notify_about_events() {
        let mut emitter = Emitter::<i32>::new();
        let counter = Arc::new(Mutex::new(Counter { value: 0 }));

        emitter.subscribe(Arc::downgrade(&counter));
        emitter.emit(10);
        emitter.emit(20);

        assert_eq!(counter.lock().unwrap().value, 30);
    }

    #[test]
    fn should_not_panic_if_handle_was_dropped() {
        let mut emitter = Emitter::<i32>::new();
        {
            let counter = Arc::new(Mutex::new(Counter { value: 0 }));
            emitter.subscribe(Arc::downgrade(&counter));
        }
        emitter.emit(10);

        // Handler should be removed
        assert_eq!(emitter.handlers.len(), 0);
    }
}
