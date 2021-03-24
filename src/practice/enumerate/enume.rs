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
