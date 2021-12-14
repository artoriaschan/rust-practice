pub fn demo_1() {
  println!("call a function!");
}

pub fn demo_2(x: i32, y: i32) {
  println!("the value of arguments is: {:?}", (x, y));
}

pub fn demo_3(x: i32) -> i32 {
  x + 1
}

pub fn demo_4() {
  let x = 5;
  println!("before call demo_3 function, the value of x is {}", x);
  let x = demo_3(x);
  println!("after call demo_3 function, the value of x is {}", x);
}

pub fn demo_5() {
  let x = 5;
  println!("before call demo_3 function, the value of x is {}", x);
  let x = demo_3(x);
  println!("after call demo_3 function, the value of x is {}", x);
}
