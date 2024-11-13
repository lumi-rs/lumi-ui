pub enum CustomEvent {
    Callback(Box<dyn FnOnce() + Send + 'static>)
}