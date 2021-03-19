pub fn demo_1() {
  let s1 = String::from("hello");
  let s2 = s1;

  // println!("{}!", s1); // 编译不通过: value borrowed here after move
  println!("{}!", s2);
}

fn takes_ownership(s: String) -> usize {
  s.len()
}

fn makes_copy(num: usize) {
  println!("In make_copy fn, s1's length is {}!", num);
}

pub fn demo_2() {
  let s1 = String::from("hello");
  let len = takes_ownership(s1);
  makes_copy(len);
  // println!("{}!", s1); // 编译不通过: value borrowed here after move
  println!("s1's length is {}!", len);
}

fn take_and_give_ownership(s1: String) -> String {
  s1
}
pub fn demo_3() {
  let s1 = String::from("hello");
  let s2 = take_and_give_ownership(s1);
  // println!("{}!", s1); // 编译不通过: value borrowed here after move
  println!("s1 is {}!", s2);
}
