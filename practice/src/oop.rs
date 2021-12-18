mod object {
  // 结构体本身被标记为pub来使其他代码可以使用自己，但其内部字段则依然保持私有。
  pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
  }

  impl AveragedCollection {
    // public
    pub fn new() -> AveragedCollection {
      AveragedCollection {
        list: vec![],
        average: 0.0,
      }
    }
    pub fn add(&mut self, value: i32) {
      self.list.push(value);
      self.update_average();
    }
    pub fn remove(&mut self) -> Option<i32> {
      let result = self.list.pop();
      match result {
        Some(value) => {
          self.update_average();
          Some(value)
        }
        None => None,
      }
    }
    pub fn average(&self) -> f64 {
      self.average
    }
    // private
    fn update_average(&mut self) {
      let total: i32 = self.list.iter().sum();
      self.average = total as f64 / self.list.len() as f64;
    }
  }
}
mod demo_1 {
  use super::object::AveragedCollection;

  pub fn main() {
    let mut ac = AveragedCollection::new();
    ac.add(1);
    ac.add(2);
    ac.add(3);
    println!("Average = {}", ac.average());

    let remove_item = ac.remove().unwrap();
    println!("Remove item = {}", remove_item);
    println!("Average = {}", ac.average());

    ac.add(4);
    println!("Average = {}", ac.average());
  }
}

mod gui {
  pub trait Draw {
    fn draw(&self);
  }
  // 如果你需要的仅仅是同质集合（homogeneous collection），那么使用泛型和trait约束就再好不过了
  pub struct Screen {
    // 创建存储 Draw trait对象的动态数组
    // 通过dyn关键字, Screen实例只会接收那些能够调用draw方法的值。

    // 动态派发
    // Rust必然会在我们使用trait对象时执行动态派发。
    // Rust会在运行时通过trait 对象内部的指针去定位具体调用哪个方法。
    // 该定位过程会产生一些不可避免的运行时开销，而这并不会出现在静态派发中。
    // 动态派发还会阻止编译器内联代码，进而使得部分优化操作无法进行。

    // 对象安全
    // 你只能把满足对象安全（object-safe）的trait转换为trait对象。
    // 如果一个trait中定义的所有方法满足下面两条规则，那么这个trait就是对象安全的：
    //  • 方法的返回类型不是Self。
    //  • 方法中不包含任何泛型参数。
    pub components: Vec<Box<dyn Draw>>,
  }

  impl Screen {
    pub fn run(&self) {
      for component in self.components.iter() {
        component.draw();
      }
    }
  }

  pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
  }

  impl Draw for Button {
    fn draw(&self) {
      // 实际绘制
      println!("draw a Button");
    }
  }

  pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
  }

  impl Draw for SelectBox {
    fn draw(&self) {
      // 实际绘制一个选择框的代码
      println!("draw a SelectBox");
    }
  }
}

mod demo_2 {
  use super::gui::{Button, Screen, SelectBox};

  pub fn main() {
    let screen = Screen {
      components: vec![
        Box::new(SelectBox {
          width: 75,
          height: 10,
          options: vec![
            String::from("Yes"),
            String::from("Maybe"),
            String::from("No"),
          ],
        }),
        Box::new(Button {
          width: 50,
          height: 10,
          label: String::from("OK"),
        }),
      ],
    };
    screen.run();
  }
}

mod demo_3 {
  struct Post {
    // Rust不允许结构体中出现未被填充的值。
    state: Option<Box<dyn State>>,
    content: String,
  }
  impl Post {
    fn new() -> Post {
      Post {
        state: Some(Box::new(Draft {})),
        content: String::new(),
      }
    }
    fn content(&self) -> &str {
      // 调用了Option的as_ref方法， 因为我们需要的只是Option中值的引用，而不是它的所有权。
      self.state.as_ref().unwrap().content(&self)
    }
    fn add_text(&mut self, text: &str) {
      self.content.push_str(text);
    }
    fn request_review(&mut self) {
      // 我们可以通过Option<T>的take方法来取出state字段的Some值，并在原来的位置留下一个None。
      if let Some(s) = self.state.take() {
        self.state = Some(s.request_review())
      }
    }
    fn approve(&mut self) {
      if let Some(s) = self.state.take() {
        self.state = Some(s.approve())
      }
    }
  }
  trait State {
    // 我们选择了self: Box<Self>来作为方法的第一个参数，而不是self、&self或&mut self。
    // 这个语法意味着该方法只能被包裹着当前类型的Box实例调用
    // 它会在调用过程中获取Box<Self>的所有权并使旧的状态失效，从而将Post的状态值转换为一个新的状态。
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn approve(self: Box<Self>) -> Box<dyn State>;
    // 默认的trait实现, 返回字符串切片
    // 这使得我们可以不必在Draft和PendingReview结构体中重复实现content。
    fn content<'a>(&self, post: &'a Post) -> &'a str {
      ""
    }
  }
  // 草稿状态
  struct Draft {}
  impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      Box::new(PendingReview {})
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }
  }
  // 等待审核
  struct PendingReview {}
  impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
      Box::new(Published {})
    }
  }
  // 发表
  struct Published {}
  impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
      self
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
      self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {
      &post.content
    }
  }

  pub fn main() {
    let mut post = Post::new();
    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    post.request_review();
    assert_eq!("", post.content());
    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
  }
}
pub fn run() {
  println!("=============run demos of oop=============");
  // demo_1::main();
  // demo_2::main();
  demo_3::main();
}
