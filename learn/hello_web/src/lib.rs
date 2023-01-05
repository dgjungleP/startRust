use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Job>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: JoinHandle<()>,
}
impl ThreadPool {
    pub fn new(capacity: usize) -> ThreadPool {
        assert!(capacity > 0);
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(capacity);
        for i in 0..capacity {
            workers.push(Worker::new(i, Arc::clone(&receiver)));
        }
        ThreadPool { workers, sender }
    }
    pub fn excute<F>(&self, function: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(function);
        self.sender.send(job).unwrap();
    }
}

impl Worker {
    fn new(id: usize, reseiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                let job = reseiver.lock().unwrap().recv().unwrap();
                println!("Woker {} got a job: executing.", id);
                job();
            }),
        }
    }
}
