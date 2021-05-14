pub struct Notifier<'a, E> {
    listeners: Vec<Box<dyn FnMut(&E) + 'a>>,
}

impl<'a, E> Notifier<'a, E> {
    pub fn new() -> Self {
        Self {
            listeners: Vec::new(),
        }
    }

    pub fn register<F>(&mut self, listener: F)
    where
        F: FnMut(&E) -> () + 'a,
    {
        self.listeners.push(Box::new(listener))
    }

    pub fn notify(&mut self, event: E) {
        for listener in &mut self.listeners {
            listener(&event)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_register_listener_and_notify_about_events() {
        let mut sum = 0;

        {
            let mut notifier = Notifier::<i32>::new();
            notifier.register(|i| sum += i);
            notifier.notify(10);
            notifier.notify(20);
        }

        assert_eq!(sum, 30);
    }
}
