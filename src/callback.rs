use std::{fmt::Debug, rc::Rc, sync::Arc};

use futures::future::BoxFuture;

#[derive(Debug, Clone)]
pub enum Callback {
    Blocking(BlockingCallback),
    Async(AsyncCallback)
}

impl Callback {
    pub fn new(closure: impl Fn() + 'static) -> Self {
        Self::Blocking(BlockingCallback::new(closure))
    }

    pub fn asynchronous(closure: impl Fn() -> BoxFuture<'static, ()> + 'static + Send + Sync) -> Self {
        Self::Async(AsyncCallback::new(closure))
    }

    pub fn run(&self) {
        match self {
            Callback::Blocking(blocking_callback) => blocking_callback.invoke(),
            Callback::Async(async_callback) => {
                let cloned = async_callback.cloned();
                crate::THREAD_POOL.spawn_ok(async move {
                    cloned.invoke().await;
                });
            },
        }
    }
}

#[derive(Clone)]
pub struct BlockingCallback {
    closure: Rc<dyn Fn()>
}

impl BlockingCallback {
    pub fn new(closure: impl Fn() + 'static) -> Self {
        Self {
            closure: Rc::new(closure)
        }
    }

    pub fn invoke(&self) {
        self.closure.as_ref()();
    }
}

impl Debug for BlockingCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BlockingCallback { ... }")
    }
}

#[derive(Clone)]
pub struct AsyncCallback {
    closure: Arc<dyn Fn() -> BoxFuture<'static, ()> + Send + Sync>
}

impl AsyncCallback {
    pub fn new(closure: impl Fn() -> BoxFuture<'static, ()> + 'static + Send + Sync) -> Self {
        Self {
            closure: Arc::new(closure)
        }
    }

    pub async fn invoke(&self) {
        self.closure.as_ref()().await;
    }

    pub fn cloned(&self) -> Self {
        Self { closure: self.closure.clone() }
    }
}

impl Debug for AsyncCallback {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("AsyncCallback { ... }")
    }
}