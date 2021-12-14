pub fn demo_1() {
  let mut s = String::from("foo");
  s.push_str("bar");
  println!("the value of s is {}", s);
  let s2 = "bar";
  s.push_str(s2);
  println!("the value of s is {}", s);
  let s3 = "log";
  let s4 = "gogo";
  let s5 = s + "-" + &s3 + "-" + &s4;
  println!("the value of s is {}", s5);
}
pub fn demo_2() -> String {
  let s1 = String::from("tic");
  let s2 = String::from("tac");
  let s3 = String::from("toe");

  let s = format!("{}-{}-{}", s1, s2, s3);
  println!("the value of s is {}", s);
  s
}
pub fn demo_3() {
  let s = demo_2();
  for c in s.chars() {
    println!("{}", c);
  }
}
