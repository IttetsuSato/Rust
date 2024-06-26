use std::{sync::{mpsc, Arc, Mutex}, thread};

// struct Job;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}


impl ThreadPool {
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // スレッドを生成してベクタに格納する
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

        self.sender.send(job).unwrap();
    }
}

trait FnBox {
  fn call_box(self: Box<Self>);
}

impl<F: FnOnce()> FnBox for F {
  fn call_box(self: Box<F>) {
      (*self)()
  }
}

type Job = Box<FnBox + Send + 'static>;

struct Worker {
  id: usize,
  thread: thread::JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
      let thread = thread::spawn(move || {
        loop {
          let job = receiver.lock().unwrap().recv().unwrap();

          println!("Worker {} got a job; executing.", id);

          (*job)();
        }
      });

      Worker {
          id,
          thread,
      }
  }
}