use super::*;
use futures::task::waker_ref;
use std::{
    sync::mpsc::Receiver,
    sync::Arc,
    task::{Context, Poll},
};

pub struct Executor {
    pub ready_queue: Receiver<Arc<Task>>, // Reciever is a channel
}

impl Executor {
    pub fn run(&self) {
        while let Ok(task) = self.ready_queue.recv() {
            let mut future_slot = task.future.lock().unwrap(); // Lock the future
            if let Some(mut future) = future_slot.take() {
                let waker = waker_ref(&task); // Create a waker from the task itself
                let context = &mut Context::from_waker(&*waker); // Create a context from the waker
                if let Poll::Pending = future.as_mut().poll(context) {
                    *future_slot = Some(future); // If the future is not ready, put it back in its slot
                }
            }
        }
    }
}
