#[derive(Debug)]
enum Coin {
  Penny,
  Nickel,
  Dime,
  Quarter,
}

fn value_in_cents(coin: &Coin) -> u32 {
  match coin {
    Coin::Penny => 1,
    Coin::Nickel => 5,
    Coin::Dime => 10,
    Coin::Quarter => 25,
  }
}
pub fn demo_1() {
  let coin = Coin::Penny;

  let value = value_in_cents(&coin);

  println!("the value of {:?} is {}", coin, value)
}
#[derive(Debug)]
enum UsState {
  Alabama,
  Alaska,
}

#[derive(Debug)]
enum Coins {
  Penny,
  Nickel,
  Dime,
  Quarter(UsState),
}

fn value_in_cents_1(coin: &Coins) -> u32 {
  match coin {
    Coins::Penny => 1,
    Coins::Nickel => 5,
    Coins::Dime => 10,
    Coins::Quarter(state) => {
      println!("State quarter from {:?}", state);
      25
    }
  }
}

pub fn demo_2() {
  let coin = Coins::Quarter(UsState::Alabama);

  let value = value_in_cents_1(&coin);

  println!("the value of {:?} is {}", coin, value)
}

fn plus_one(x: Option<i32>) -> Option<i32> {
  match x {
    None => None,
    Some(i) => Some(i + 1),
  }
}

pub fn demo_3() {
  let five = Some(5);
  let six = plus_one(five);
  println!("the value of six is {:?}", six);
  let none = plus_one(None);
  println!("the value of none is {:?}", none);
}

pub fn demo_4() {
  let some_u8_value = 0u8;

  match some_u8_value {
    1 => println!("one!"),
    3 => println!("three!"),
    5 => println!("five!"),
    7 => println!("seven!"),
    _ => println!("other!"),
  }
}

pub fn demo_5() {
  let some_u8_value = Some(3);

  if let Some(3) = some_u8_value {
    println!("three!");
  } else {
    println!("other!");
  }
}
