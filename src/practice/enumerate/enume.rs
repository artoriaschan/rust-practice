#[derive(Debug)]
enum IpAddrKind {
  V4,
  V6,
}

#[derive(Debug)]
struct IpAddr {
  kind: IpAddrKind,
  address: String,
}

pub fn demo_1() {
  let home = IpAddr {
    kind: IpAddrKind::V4,
    address: String::from("127.0.0.1"),
  };

  let loopback = IpAddr {
    kind: IpAddrKind::V6,
    address: String::from("::1"),
  };

  println!("the value of home IpAddr is {:?}", home);
  println!("the value of loopback IpAddr is {:?}", loopback);
}

#[derive(Debug)]
enum IpAddrStr {
  V4(String),
  V6(String),
}

pub fn demo_2() {
  let home = IpAddrStr::V4(String::from("127.0.0.1"));
  let loopback = IpAddrStr::V6(String::from("::1"));

  println!("the value of home IpAddrStr is {:?}", home);
  println!("the value of loopback IpAddrStr is {:?}", loopback);
}

#[derive(Debug)]
enum IpAddrNum {
  V4(u8, u8, u8, u8),
  V6(String),
}

pub fn demo_3() {
  let home = IpAddrNum::V4(127, 0, 0, 1);
  let loopback = IpAddrNum::V6(String::from("::1"));

  println!("the value of home IpAddrNum is {:?}", home);
  println!("the value of loopback IpAddrNum is {:?}", loopback);
}

#[derive(Debug)]
enum Message {
  Quit,
  Move { x: i32, y: i32 },
  Write(String),
  ChangeColor(i32, i32, i32),
}

impl Message {
  fn call(&self) {
    println!("{:?}", self)
  }
}

pub fn demo_4() {
  let m = Message::Write(String::from("hello"));

  m.call();
}
