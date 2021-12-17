pub mod demo_1 {
  // Mutex<T>是一种智能指针
  use std::sync::Mutex;

  pub fn main() {
    let m = Mutex::new(5);
    {
      // 对lock的调用会返回一个名为MutexGuard的智能指针
      let mut num = m.lock().unwrap();
      *num = 6
      // 释放锁会发生在内部作用域的结尾处
    }
    println!("m = {:?}", m);
  }
}

pub mod demo_2 {
  // example 2:
  // use std::rc::Rc;
  use std::sync::{Arc, Mutex};
  use std::thread;
  pub fn main() {
    // example 2:
    // let counter = Rc::new(Mutex::new(0));
    // Arc 智能指针, 既拥有类似于 Rc<T>的行为，又保证了自己可以被安全地用于并发场景
    // 原子和原生类型的用法十分相似，并且可以安全地在多个线程间共享
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
      // example 2:
      // Rc 无法安全地在线程间传递
      // let counter = Rc::clone(&counter);
      // let handle = thread::spawn(move || {
      //   let mut num = counter.lock().unwrap();
      //   *num += 1
      // });
      // example 1:
      // 不应该将counter的所有权移动到多个线程中
      // let handle = thread::spawn(move || {
      //   let mut num = counter.lock().unwrap();
      //   *num += 1
      // });
      //
      let counter = Arc::clone(&counter);
      let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1
      });
      handles.push(handle);
    }

    for handle in handles {
      handle.join().unwrap();
    }

    println!("Result: {}", counter.lock().unwrap());
  }
}
