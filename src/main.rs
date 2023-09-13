use asynclib_rs::{new_executor_and_spawner, TimerFuture};
use std::time::Duration;

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    for task in 0..1000 {
        spawner.spawn(async move {
            TimerFuture::new(Duration::new(1, 0)).await;
            println!("Task-ID {} completed...", task);
        });
    }
    drop(spawner);
    executor.run();
}
