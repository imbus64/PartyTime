use asynclib_rs::{new_executor_and_spawner, task::sleep, TimerFuture};
use std::time::Duration;
const INTERVAL: i32 = 100;

async fn regular_async_fn() {
    sleep(Duration::new(2, 0)).await;
    println!("Regular async fn");
}

async fn nothing() {}

fn main() {
    let (executor, spawner) = new_executor_and_spawner();
    for task_id in 1..=1000 {
        spawner.spawn(async move {
            TimerFuture::new(Duration::new(1, (task_id * 1000000) as u32)).await;
            if (task_id) % INTERVAL == 0 {
                println!("Task-ID {}..{} completed...", task_id - INTERVAL, task_id);
                nothing().await; // Use await as usual
            }
        });
    }
    spawner.spawn(regular_async_fn());
    drop(spawner);
    executor.run();
}
