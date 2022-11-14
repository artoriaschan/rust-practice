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
        let received = rx.recv().unwrap();
        println!("Got: {}", received);
        // try_recv方法不会阻塞线程，它会立即返回Result<T, E>
        // let received = rx.try_recv();
        // println!("receive {:?}", received);
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
        // Receiver实现了 Iterator 和 IntoIterator
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

pub mod demo_4 {
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;
    pub fn main() {
        // 同步通道发送消息是阻塞的，只有在消息被接收后才解除阻塞
        // This channel has an internal buffer on which messages will be queued.
        // `bound` specifies the buffer size.
        let (tx, rx) = mpsc::sync_channel(1);

        let handle = thread::spawn(move || {
            // println!("发送之前");
            // tx.send(1).unwrap();
            // println!("发送之后");
            println!("首次发送之前");
            tx.send(1).unwrap();
            println!("首次发送之后");
            tx.send(1).unwrap();
            println!("再次发送之后");
        });

        println!("睡眠之前");
        thread::sleep(Duration::from_secs(3));
        println!("睡眠之后");

        println!("receive {}", rx.recv().unwrap());
        handle.join().unwrap();
    }
}

pub mod demo_5 {
    use std::sync::mpsc::{self, Receiver, Sender};

    // 使用枚举类型来实现不同类型消息同一个通道
    // Rust 会按照枚举中占用内存最大的那个成员进行内存对齐，易造成内存浪费
    enum Fruit {
        Apple(u8),
        Orange(String),
    }

    pub fn main() {
        // 消息类型为枚举
        let (tx, rx): (Sender<Fruit>, Receiver<Fruit>) = mpsc::channel();

        tx.send(Fruit::Orange("sweet".to_string())).unwrap();
        tx.send(Fruit::Apple(2)).unwrap();

        for _ in 0..2 {
            match rx.recv().unwrap() {
                Fruit::Apple(count) => println!("received {} apples", count),
                Fruit::Orange(flavor) => println!("received {} oranges", flavor),
            }
        }
    }
}

pub mod demo_6 {
    use std::sync::mpsc;
    use std::thread;

    pub fn main() {
        let (send, recv) = mpsc::channel();
        let num_threads = 3;
        for i in 0..num_threads {
            // clone sender
            let thread_send = send.clone();
            thread::spawn(move || {
                thread_send.send(i).unwrap();
                println!("thread {:?} finished", i);
            });
        }

        // 通道关闭的两个条件：发送者全部drop或接收者被drop
        // send不手动drop，只能等主线程执行结束才隐式drop
        // 这导致 recv for 循环永远无法结束
        // 在这里drop send...
        drop(send);

        for x in recv {
            println!("Got: {}", x);
        }
        println!("finished iterating");
    }
}
