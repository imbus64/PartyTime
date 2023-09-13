pub mod executor;
pub mod spawner;
pub mod task;
pub mod timerfuture;

pub use executor::Executor;
pub use spawner::new_executor_and_spawner;
pub use spawner::Spawner;
pub use task::Task;
pub use timerfuture::TimerFuture;

use futures::{
    future::{BoxFuture, FutureExt},
    task::ArcWake,
};

use std::{
    future::Future,
    pin::Pin,
    sync::mpsc::{sync_channel, SyncSender},
    sync::{Arc, Mutex},
    task::{Context, Poll, Waker},
    thread,
    time::Duration,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn async_stress() {
        let (executor, spawner) = new_executor_and_spawner();
        for _ in 0..1000 {
            spawner.spawn(async {
                TimerFuture::new(Duration::new(1, 0)).await;
                assert!(true)
            });
        }
        drop(spawner);
        executor.run();
    }
}
