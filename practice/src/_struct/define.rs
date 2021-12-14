#[derive(Debug)]
struct User {
  username: String,
  email: String,
  sign_in_count: u64,
  active: bool,
}

fn generate_user(username: String, email: String) -> User {
  User {
    // field init shorthand (字段初始化简写)
    username,
    email,
    sign_in_count: 1,
    active: true,
  }
}
pub fn demo_1() {
  let user = generate_user(
    String::from("someusername123"),
    String::from("someusername123@example.com"),
  );

  println!("the struct of user is {:#?}", user)
}

pub fn demo_2() {
  let mut user = generate_user(
    String::from("someusername123"),
    String::from("someusername123@example.com"),
  );

  println!("the struct of user is {:#?}", user);

  user.username = String::from("someusername456");
  user.email = String::from("someusername456@example.com");

  println!("the struct of user is {:#?}", user);
}

pub fn demo_3() {
  let user1 = generate_user(
    String::from("someusername123"),
    String::from("someusername123@example.com"),
  );

  println!("the struct of user1 is {:#?}", user1);

  let user2 = User {
    username: String::from("someusername456"),
    ..user1
  };

  println!("the struct of user2 is {:#?}", user2);
}
#[derive(Debug)]
struct Color(i32, i32, i32);
#[derive(Debug)]
struct Point(i32, i32, i32);

pub fn demo_4() {
  let black = Color(0, 0, 0);
  let origin = Point(0, 0, 0);

  println!("the value of black is {:#?}", black);
  println!("the value of origin is {:#?}", origin);
}
