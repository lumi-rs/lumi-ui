use std::sync::{LazyLock, OnceLock};

use crossbeam_channel::Sender;
use custom_event::CustomEvent;
use futures::executor::ThreadPool;
use log::{error, info};
use lumi2d::types::Event;

pub mod macros;

pub mod backend;
pub mod custom_event;
pub mod elements;
pub mod widgets;
pub mod signals;


pub(crate) static GLOBAL_SENDER: OnceLock<Sender<Event<CustomEvent>>> = OnceLock::new(); 

pub static THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| {
    let pool = ThreadPool::builder()
    .name_prefix("lumi-pool-")
    .create()
    .unwrap();

    info!("Thread pool created.");

    pool
});

pub(crate) fn global_send(event: Event<CustomEvent>) {
    if let Some(sender) = GLOBAL_SENDER.get() {
        sender.send(event)
        .map_err(|err| error!("Failed to global send an event: {}", err)).ok();
    }
}