pub fn demo_1() {
  let x = 3;
  if x == 3 {
    println!("number was three!");
  } else {
    println!("number wasn't three!");
  };
}

pub fn demo_2(x: i32) {
  if x % 4 == 0 {
    println!("the number is divisible by 4!");
  } else if x % 3 == 0 {
    println!("the number is divisible by 3!");
  } else if x % 2 == 0 {
    println!("the number is divisible by 2!");
  } else {
    println!("the number is divisible by 4, 3, or 2!");
  };
}

pub fn demo_3() {
  let condition = true;
  let x = if condition { 10 } else { 11 };
  println!("the value of x is {}", x);
}
