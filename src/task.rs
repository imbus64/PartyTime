use super::*;

pub struct Task {
    pub(crate) future: Mutex<Option<BoxFuture<'static, ()>>>, // Mutex to share the future between the executor and the spawner
    pub(crate) task_sender: SyncSender<Arc<Task>>,            // SyncSender is a channel
}

impl ArcWake for Task {
    fn wake_by_ref(arc_self: &Arc<Self>) {
        let cloned = arc_self.clone();
        arc_self
            .task_sender
            .send(cloned)
            .expect("too many tasks queued");
    }
}
