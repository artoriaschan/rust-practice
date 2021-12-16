// 内部可变性（interior mutability）是Rust的设计模式之一，它允许你在只持有不可变引用的前提下对数据进行修改。
// 为了能够改变数据，内部可变性模式在它的数据结构中使用了unsafe（不安全）代码来绕过Rust正常的可变性和借用规则。

// RefCell<T>并没有完全绕开借用规则：我们虽然使用内部可变性通过了编译阶段的借用检查，但借用检查的工作仅仅是被延后到了运行阶段。
// 如果你违反了借用规则，那么就会得到一个panic! 而不再只是编译时的错误。

pub mod lib {
  pub trait Messenger {
    fn send(&self, msg: &str);
  }

  pub struct LimitTracker<'a, T: 'a + Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
  }

  impl<'a, T> LimitTracker<'a, T>
  where
    T: Messenger,
  {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
      LimitTracker {
        messenger,
        value: 0,
        max,
      }
    }
    pub fn set_value(&mut self, value: usize) {
      self.value = value;

      let percentage_of_max = self.value as f64 / self.max as f64;

      if percentage_of_max >= 1.0 {
        self.messenger.send("Error: You are over your quota!");
      } else if percentage_of_max >= 0.9 {
        self
          .messenger
          .send("Urgent warning: You've used up over 90% of your quota!");
      } else if percentage_of_max >= 0.75 {
        self
          .messenger
          .send("Warning: You've used up over 75% of your quota!");
      }
    }
  }
}

#[cfg(test)]
pub mod tests {
  use super::lib::{LimitTracker, Messenger};
  use std::cell::RefCell;

  struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,
  }
  impl MockMessenger {
    fn new() -> MockMessenger {
      MockMessenger {
        sent_messages: RefCell::new(vec![]),
      }
    }
  }

  impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      // 同一个作用域中不允许出现两个可变引用。
      // let mut one_borrow = self.sent_messages.borrow_mut();
      // let mut two_borrow = self.sent_messages.borrow_mut();
      self.sent_messages.borrow_mut().push(String::from(message));
    }
  }

  #[test]
  fn it_sends_an_over_75_percent_warning_message() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
    // 我们会在创建不可变和可变引用时分别使用语法&与&mut。
    // 对于 RefCell<T>而言，我们需要使用borrow与borrow_mut方法来实现类似的功能，这两者都被作为RefCell<T>的安全接口来提供给用户。
    assert_eq!(mock_messenger.sent_messages.borrow_mut().len(), 1);
  }
}

pub mod demo_1 {
  #[derive(Debug)]
  enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
  }

  use std::cell::RefCell;
  use std::rc::Rc;
  use List::{Cons, Nil};

  pub fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(6)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(10)), Rc::clone(&a));

    *(value.borrow_mut()) += 10;
    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
  }
}

pub mod demo_2 {
  use std::cell::RefCell;
  use std::rc::Rc;
  use List::{Cons, Nil};

  #[derive(Debug)]
  enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
  }

  impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
      match self {
        Cons(_, item) => Some(item),
        Nil => None,
      }
    }
  }

  pub fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());
    // b指向a
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());
    if let Some(link) = a.tail() {
      // 将a指向的下一个元素Nil修改为b来创建出循环
      *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));
    // 取消下面的注释行便可以观察到循环引用；它会造成栈的溢出。
    // println!("a next item = {:?}", a.tail());
  }
}

// 强引用可以被我们用来共享一个Rc<T>实例的所有权，而弱引用则不会表达所有权关系。
// 一旦强引用计数减为0，任何由弱引用组成的循环就会被打破。因此，弱引用不会造成循环引用。
pub mod demo_3 {
  use std::cell::RefCell;
  use std::rc::{Rc, Weak};

  #[derive(Debug)]
  struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
  }
  // 父节点自然应该拥有子节点的所有权，但子节点却不应该拥有父节点
  // 这正是应当使用弱引用的场景
  pub fn main() {
    let leaf = Rc::new(Node {
      value: 3,
      parent: RefCell::new(Weak::new()),
      children: RefCell::new(vec![]),
    });
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
    );
    // 块中创建分支节点
    {
      let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // leaf 增加强引用
      });
      // branch增加弱引用
      *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

      println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch),
      );

      println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf),
      );
    } // leaf 强引用减1

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
      "leaf strong = {}, weak = {}",
      Rc::strong_count(&leaf),
      Rc::weak_count(&leaf),
    );
  }
}
