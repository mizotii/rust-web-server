use std::{error::Error, fmt, thread::{self, JoinHandle}, sync::{mpsc, Arc, Mutex}};

//
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

pub struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;
//

#[derive(Debug)]
pub struct PoolCreationError;

//
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
            let (sender, receiver) = mpsc::channel();

            let receiver = Arc::new(Mutex::new(receiver));

            let mut workers = Vec::with_capacity(size);

            for id in 0..size {
                workers.push(Worker::new(id, Arc::clone(&receiver)));
            }

            Ok(ThreadPool { workers, sender })
        } else {
            Err(PoolCreationError)
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });
        
        Worker { id, thread }
    }
}
//

impl Error for PoolCreationError {}

impl fmt::Display for PoolCreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "can't create a thread pool!")
    }
}