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
pub mod byte_source;
pub mod callback;


pub static LOADING_COLOR: u32 = 0x57595C66;

pub(crate) static GLOBAL_SENDER: OnceLock<Sender<Event<CustomEvent>>> = OnceLock::new();

#[cfg(feature = "reqwest")]
pub(crate) static REQWEST_CLIENT: LazyLock<reqwest::Client> = LazyLock::new(reqwest::Client::new);
#[cfg(feature = "ureq")]
pub(crate) static UREQ_CLIENT: LazyLock<ureq::Agent> = LazyLock::new(ureq::Agent::new);

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