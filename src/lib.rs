use std::sync::LazyLock;

use futures::executor::{LocalPool, ThreadPool};
use log::info;

pub mod macros;

pub mod backend;
pub mod elements;
pub mod widgets;
pub mod signals;

pub static THREAD_POOL: LazyLock<ThreadPool> = LazyLock::new(|| {
    let pool = ThreadPool::builder()
    .name_prefix("lumi-pool-")
    .create()
    .unwrap();

    info!("Thread pool created.");

    pool
});

thread_local! {
    pub static LOCAL_POOL: LazyLock<LocalPool> = LazyLock::new(|| {
        LocalPool::new()
    });
}

