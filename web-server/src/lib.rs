use std::sync::mpsc;
use std::sync::Arc;
use std::sync::Mutex;
use std::thread;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Message>,
}

// 类型别名
type Job = Box<dyn FnOnce() + Send + 'static>;

enum Message {
  NewJob(Job),
  Terminate,
}

impl ThreadPool {
  /// 创建线程池
  ///
  /// 线程池中线程的数量
  ///
  /// # Panics
  ///
  /// `new`函数会在size为0时触发panic.
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();
    let receiver = Arc::new(Mutex::new(receiver));
    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      // 创建线程并且存储在线程池中
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&self, f: F)
  where
    // 查看spawn的定义 F: FnOnce() -> T + Send + 'static,
    F: FnOnce() + Send + 'static,
  {
    let job = Box::new(f);
    self.sender.send(Message::NewJob(job)).unwrap();
  }
}

impl Drop for ThreadPool {
  fn drop(&mut self) {
    println!("Sending terminate message to all workers.");

    for _ in &mut self.workers {
      self.sender.send(Message::Terminate).unwrap();
    }

    println!("Shutting down all workers.");

    for worker in &mut self.workers {
      println!("Shutting down worker {}", worker.id);
      // 为Option值调用take方法会将Some变 体的值移出并在原来的位置留下None变体。
      if let Some(thread) = worker.thread.take() {
        thread.join().unwrap();
      }
    }
  }
}

struct Worker {
  id: usize,
  // 如果Worker持有的是一个Option<thread::JoinHandle<()>>，
  // 那么我们就可以在Option上调用take方法来将Some变体的值移出来，并在原来的位置留下None变体。
  thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
    let thread = thread::spawn(move || {
      loop {
        // lock方法来请求互斥锁
        // 调用recv来从通道中接收Job
        let message = receiver.lock().unwrap().recv().unwrap();
        match message {
          Message::NewJob(job) => {
            println!("Worker {} got a job; executing.", id);
            job();
          }
          Message::Terminate => {
            println!("Worker {} was told to terminate.", id);
            break;
          }
        }
      }
    });

    Worker {
      id,
      thread: Some(thread),
    }
  }
}
