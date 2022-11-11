pub mod demo_1 {
    use std::thread;
    use std::time::Duration;

    pub fn main() {
        // 通过将thread::spawn返回的结果保存在一个变量中，来避免新线程出现不执行或不能完整执行的情形。
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        // 放在这里，主线程会等待新线程执行完毕后才开始执行自己的for循环。
        // handle.join().unwrap();
        // 只要这段程序中的主线程运行结束，创建出的新线程就会相应停止，而不管它的打印任务是否完成。
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        // 调用它的join方法可以阻塞当前线程直到对应的新线程运行结束。
        handle.join().unwrap();
    }
}
pub mod demo_2 {
    use std::thread;

    pub fn main() {
        let v = vec![1, 2, 3, 4];
        // move 会强制闭包获得它所需值的所有权，而不仅仅是基于Rust的推导来获得值的借用。
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
        });

        handle.join().unwrap();
    }
}
// 只被调用一次的函数
pub mod demo_3 {
    use std::sync::Once;
    use std::thread;

    static mut VAL: usize = 0;
    static INIT: Once = Once::new();

    pub fn main() {
        let handle1 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 1;
            });
        });

        let handle2 = thread::spawn(move || {
            INIT.call_once(|| unsafe {
                VAL = 2;
            });
        });

        handle1.join().unwrap();
        handle2.join().unwrap();

        println!("{}", unsafe { VAL });
    }
}

// 用条件控制线程的挂起和执行
pub mod demo_4 {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    pub fn main() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = pair.clone();

        thread::spawn(move || {
            let &(ref lock, ref cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            println!("changing started");
            *started = true;
            cvar.notify_one();
        });

        let &(ref lock, ref cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        while !*started {
            started = cvar.wait(started).unwrap();
        }

        println!("started changed");
    }
}
