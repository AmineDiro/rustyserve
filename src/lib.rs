use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

impl Worker {
    fn build<F>(id: usize, work_queue: Arc<Mutex<Vec<Box<F>>>>) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Worker {
            id,
            thread: thread::spawn(move || {
                // todo: get mutex Pop work from work_queue and execute it
                loop {
                    let mut work_queue = work_queue.lock().unwrap();
                    if let Some(func) = work_queue.pop() {
                        drop(work_queue);
                        func()
                    }
                }
            }),
        }
    }
}
pub struct ThreadPool<F>
where
    F: FnOnce() + Send + 'static,
{
    pub size: usize,
    work_queue: Arc<Mutex<Vec<Box<F>>>>,
    _workers: Vec<Worker>,
}

impl<F> ThreadPool<F>
where
    F: FnOnce() + Send + 'static,
{
    pub fn new(size: usize) -> Self {
        assert!(size > 0);
        let work_queue: Arc<Mutex<Vec<Box<F>>>> = Arc::new(Mutex::new(Vec::new()));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            let work_queue = Arc::clone(&work_queue);
            workers.push(Worker::build(id, work_queue));
        }

        ThreadPool {
            size,
            work_queue,
            _workers: workers,
        }
    }

    pub fn execute(&mut self, func: F) {
        let mut worker_queue = self.work_queue.lock().unwrap();
        worker_queue.push(Box::new(func))
    }
}
