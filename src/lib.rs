use std::{error::Error, fmt, thread};

#[derive(Debug)]
pub struct ThreadPool {
    threads: Vec<thread::JoinHandle<()>>,
}
#[derive(Debug)]
pub struct PoolCreationError;

impl ThreadPool {
    /// Creates a new thread pool.
    /// 
    /// size: The number of threads in the pool.
    /// 
    /// # Panics
    /// 
    /// `build` function returns an error if size is not positive.
    pub fn build(size: usize) -> Result<ThreadPool, PoolCreationError> {
        if size > 0 {
            let mut threads = Vec::with_capacity(size);
            for _ in 0..size {
                // todo: create threads and store in vector
            }

            Ok(ThreadPool { threads })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {

    }
}

impl Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can't create a thread pool!")
    }
}