use std::{
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread::{self, JoinHandle},
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Sender<Message>,
}
type Job = Box<dyn FnOnce() + Send + 'static>;
enum Message {
    NewJob(Job),
    Terminate,
}

struct Worker {
    id: usize,
    thread: Option<JoinHandle<()>>,
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}

impl Worker {
    fn new(id: usize, reseiver: Arc<Mutex<Receiver<Message>>>) -> Worker {
        Worker {
            id,
            thread: Some(thread::spawn(move || loop {
                let message = reseiver.lock().unwrap().recv().unwrap();
                match message {
                    Message::NewJob(job) => {
                        println!("Woker {} got a job: executing.", id);
                        job();
                    }
                    Message::Terminate => {
                        println!("Woker {} was told to terminate.", id);
                        break;
                    }
                }
            })),
        }
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap()
            };
        }
    }
}
