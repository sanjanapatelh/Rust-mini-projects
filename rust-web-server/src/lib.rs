use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize , receiver:Arc<Mutex<mpsc::Receiver<job>>>) -> Worker {
        let thread = thread::spawn( move || loop{
            // Concurrency Primitives: Arc and Mutex 
            // 1. receiver.lock(): Acquires the Mutex lock, ensuring exclusive access.
            // 2. unwrap(): Panics if the thread holding the lock crashed (we keep this for simplicity here).
            // 3. recv(): Blocks the worker thread until a job is sent down the channel.
            let job = receiver.lock().unwrap().recv().unwrap();

            println!(" Worker {} got a job ; executing, " , id);

            job()

        });

        Worker { id ,thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<job>, // send job to the eorkers 
}

impl ThreadPool {
    // Creates a new ThreadPool. Panics if size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!( size > 0 );

        let (sender , receiver) = mpsc::channel();

        // 🌟 Arc<Mutex<T>>: Allows multiple workers (threads) to safely share 
        // a single mutable resource (the receiver end of the channel).

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size{
            // Give each worker a shared, cloneable reference to the receiver
            workers.push(Worker::new(id,Arc::clone(&receiver)));
        }

        ThreadPool { workers , sender }

    }

    /// Sends a closure (a job) to a worker thread for execution.

    pub fn execute<F>(&self , f:F)
    where 
        F: FnOnce() + Send + 'static,
        {
            let job = Box::new(f);

            // Send the job down the channel.
            self.sender.send(job).unwrap();
        } 
    


}