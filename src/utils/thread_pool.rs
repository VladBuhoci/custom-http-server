use std::sync::{Arc, mpsc, Mutex};
use std::thread;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Self {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .unwrap()
                .recv()
                .unwrap();

            match message {
                Message::NewJob(job) => {
                    println!("Worker [{}] got a new job! Executing job...", id);
                    job();
                }

                Message::Terminate => {
                    println!("Worker [{}] was told to terminate! Stopping listening to new jobs...", id);
                    break;
                }
            }
        });

        Self {
            id,
            thread: Some(thread),
        }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}

impl ThreadPool {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "Thread pool capacity cannot be zero or less than zero!");

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(capacity);

        for index in 0..capacity {
            workers.push(Worker::new(index, Arc::clone(&receiver)));
        }

        Self {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static
    {
        let job = Box::new(f);

        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending 'Terminate' message to all workers.");

        // We have no control over which worker gets the message first, second, third...
        // But we know how many workers we have and that each worker will eventually receive the message.
        // So it is enough to send N 'Terminate' messages, where N is the number of workers.
        for _ in &self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down all workers.");

        // Wait for workers to end their current jobs (and process the 'terminate' signal/message).
        for worker in &mut self.workers {
            println!("Waiting for worker '{}' to shut down...", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }

            println!("Worker '{}' has been shut down.", worker.id);
        }
    }
}
