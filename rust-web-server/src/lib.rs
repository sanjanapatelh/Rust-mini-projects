use std::thread;
use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;

type job = Box<dyn FnOnce() + Send + 'static>;


// Enum messgaes sent over channel
enum Message {
    NewJob(job),
    Terminate,
}


struct Worker {
    id: usize,
    thread: Option< thread::JoinHandle<()>>,
}



impl Worker {
    fn new(id: usize , receiver:Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn( move || loop{
            
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    
                    println!("Worker {} got a job; executing.", id );
                    job();
                }

                // Terminate Signal 

                Message::Terminate => {
                    println!("Worker {} was told to terminate.", id);
                    break; // Exists the loop, allowing the thread to finish
                }
            }
           

        });

        Worker { id ,thread: Some(thread) }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>, // send job to the eorkers 
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
            self.sender.send(Message::NewJob(job)).unwrap();
        } 
    


}

impl Drop for ThreadPool {

    fn drop(&mut self) {
        println!("Sending terminate messgae to all workers. ");

        for _ in &self.workers {

            self.sender.send(Message::Terminate).unwrap(); // error would only occur if the receiver has already shut down,

        }

        println!("Shutting down all workers.");

        // 2. Iterate through all workers and call join() 

        for worker in &mut self.workers{
            println!("Shutting down worker {}", worker.id);

            if let Some(thread) = worker.thread.take() { 
                 // JoinHandle::join(): Blocks the current thread until the target thread (the worker) finishes.
                thread.join().unwrap();
            }

        }



    }
    
}