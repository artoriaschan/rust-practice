pub fn demo_1() {
  let mut x = 5;
  println!("this value of x is: {}", x);
  x = 6;
  println!("this value of x is: {}", x);
}
pub fn demo_2() {
  let x = 5;
  let x = x + 1;
  let x = x * 2;
  println!("this value of x is: {}", x);
}
pub fn demo_3() {
  const MAX_POINTS: u32 = 100_000;
  println!("this value of MAX_POINTS is: {}", MAX_POINTS);
}

pub fn demo_4() {
  let spaces = "    ";
  // shadow
  let spaces = spaces.len();
  println!("this value of spaces is: {}", spaces);
}
