pub fn demo_1() {
  let a: i8 = 1;
  let b: i16 = 2;
  let c: i32 = 3;
  let d: isize = 4;
  println!("the value of a, b, c, d is: {}, {}, {}, {}!", a, b, c, d);
  let a: u8 = 10;
  let b: u16 = 11;
  let c: u32 = 12;
  let d: usize = 13;
  println!("the value of a, b, c, d is: {}, {}, {}, {}!", a, b, c, d);

  let e = 14.0; // f64
  let f: f32 = 15.0; // f32

  let sum = 16 + 17;
  let difference = 18 - 19;
  let product = 20 * 21;
  let quotient = 22 / 23;
  let remainder = 24 % 25;
  println!(
    "the operation result is {} {} {} {} {}",
    sum, difference, product, quotient, remainder
  );
}
pub fn demo_2() {
  // 元组
  let tup: (i32, f64, u8) = (500, 6.4, 1);
  let (x, y, z) = tup;
  println!("the value of y is: {}", y);
  println!("the value of x is equal tup.0: {}", x == tup.0);

  let array_1 = [1, 2, 3, 4, 5];
  let array_2: [i32; 5] = [1, 2, 3, 4, 5];
  let array_3 = [3; 5];
  println!("the value of array_3 is {:?}", array_3);
}
