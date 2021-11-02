use std::thread;
use std::sync::{Arc, Mutex, mpsc};
use log::debug;

#[allow(dead_code)]
enum Message {
    NewJob(Job),
    Terminate,
}

trait FnBox{
    fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
    fn call_box(self: Box<F>) {
        (*self)()
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}


impl ThreadPool {

    pub fn new(count: usize) -> ThreadPool {
        assert!(count > 0);

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(count);

        for id in 0..count {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool {
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

struct Worker {
    id: usize, 
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv().unwrap();

            match message {
                Message::NewJob(job) => {
                    debug!("Worker {} executing.", id);
                    job();    
                }
                Message::Terminate => {
                    debug!("Worker {} terminating.", id);
                    break;

                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}