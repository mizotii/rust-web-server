use std::{error::Error, fmt};

#[derive(Debug)]
pub struct ThreadPool;
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
            Ok(ThreadPool)
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