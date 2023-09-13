use super::*;

#[derive(Clone)]
pub struct Spawner {
    task_sender: SyncSender<Arc<Task>>, // SyncSender is a channel
}

impl Spawner {
    pub fn spawn(&self, future: impl Future<Output = ()> + 'static + Send) {
        let future = future.boxed(); // Box the future
        let task = Arc::new(Task {
            future: Mutex::new(Some(future)), // Mutex to share the future between the executor and the spawner
            task_sender: self.task_sender.clone(),
        });
        self.task_sender.send(task).expect("too many tasks queued");
    }
}

pub fn new_executor_and_spawner() -> (Executor, Spawner) {
    const MAX_QUEUED_TASKS: usize = 10_000;
    let (task_sender, ready_queue) = sync_channel(MAX_QUEUED_TASKS); // Create a channel with a capacity of 10,000 tasks
    (Executor { ready_queue }, Spawner { task_sender })
}
