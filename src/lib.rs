use std::sync::{Arc, Mutex};
use std::thread::{self, JoinHandle};
pub struct ThreadPool<F>
where
    F: FnOnce() + Send + 'static,
{
    pub size: usize,
    work_queue: Arc<Mutex<Vec<Box<F>>>>,
    _workers: Vec<JoinHandle<()>>,
}

impl<F> ThreadPool<F>
where
    F: FnOnce() + Send + 'static,
{
    pub fn new(size: usize) -> Self {
        let work_queue: Arc<Mutex<Vec<Box<F>>>> = Arc::new(Mutex::new(Vec::new()));
        let mut workers = Vec::new();
        for _i in 0..size {
            let work_queue = Arc::clone(&work_queue);
            workers.push(thread::spawn(move || {
                // todo: get mutex Pop work from work_queue and execute it
                loop {
                    let mut work_queue = work_queue.lock().unwrap();
                    if let Some(func) = work_queue.pop() {
                        drop(work_queue);
                        func()
                    }
                }
            }));
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
