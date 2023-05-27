pub struct ThreadPool {}

impl ThreadPool {
    pub fn new(n_threads: usize) -> Self {
        ThreadPool {}
    }

    pub fn execute<F>(&self, func: F)
    where
        F: FnOnce() + Send + 'static,
    {
        todo!()
    }
}
