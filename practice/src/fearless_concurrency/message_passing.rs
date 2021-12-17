pub mod demo_1 {
  // multiple producer, single consumer - 多个生产者，单个消费者
  use std::sync::mpsc;
  use std::thread;

  pub fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
      let val = String::from("hi");
      tx.send(val).unwrap();
      // 此时再调用val会报错, 因为这个词通过send方法已经发送给另外一个线程, 所有权变更.
      // println!("val is {}", val);
    });
    // 通道的接收端有两个可用于获取消息的方法：recv和try_recv。
    // 我们使用的recv（也就是receive的缩写）会阻塞主线程的执行直到有值被传入通道。
    // try_recv方法不会阻塞线程，它会立即返回Result<T, E>
    let received = rx.recv().unwrap();
    println!("Got: {}", received);
  }
}

pub mod demo_2 {
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;
  pub fn main() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ];
      for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
    });
    // 将rx视作迭代器
    // 主线程是在等待接收新线程中传递过来的值
    for received in rx {
      println!("Got: {}", received);
    }
  }
}

pub mod demo_3 {
  use std::sync::mpsc;
  use std::thread;
  use std::time::Duration;
  pub fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = mpsc::Sender::clone(&tx);
    // 线程1
    thread::spawn(move || {
      let vals = vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
      ];

      for val in vals {
        tx1.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
    });
    // 线程2
    thread::spawn(move || {
      let vals = vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
      ];

      for val in vals {
        tx.send(val).unwrap();
        thread::sleep(Duration::from_secs(1));
      }
    });
    for received in rx {
      println!("Got: {}", received);
    }
  }
}
