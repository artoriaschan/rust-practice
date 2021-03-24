#[derive(Debug)]
struct Rectangle {
  width: u32,
  height: u32,
}
fn area(rectangle: &Rectangle) -> u32 {
  rectangle.width * rectangle.height
}
pub fn demo_1() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "the area of the rectangle is {} square pixels!",
    area(&rect1)
  );
}

impl Rectangle {
  fn area(&self) -> u32 {
    self.width * self.height
  }
}

pub fn demo_2() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };

  println!(
    "the area of the rectangle {:?} is {} square pixels!",
    rect1,
    rect1.area()
  );
}

impl Rectangle {
  fn can_hold(&self, other: &Rectangle) -> bool {
    self.width >= other.width && self.height >= other.height
  }
}

pub fn demo_3() {
  let rect1 = Rectangle {
    width: 30,
    height: 50,
  };
  let rect2 = Rectangle {
    width: 10,
    height: 30,
  };
  let rect3 = Rectangle {
    width: 40,
    height: 50,
  };

  println!(
    "the rect {:?} can hold other {:?}? {}",
    rect1,
    rect2,
    rect1.can_hold(&rect2)
  );
  println!(
    "the rect {:?} can hold other {:?}? {}",
    rect1,
    rect3,
    rect1.can_hold(&rect3)
  );
}

impl Rectangle {
  fn square(size: u32) -> Rectangle {
    Rectangle {
      width: size,
      height: size,
    }
  }
}

pub fn demo_4() {
  let square = Rectangle::square(30);

  println!("the struct of square rectangle is {:?}", square);
  println!("the area of the square rectangle is {}", square.area());
}
