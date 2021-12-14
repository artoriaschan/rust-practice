fn calculate_length(s: &String) -> usize {
  s.len()
}

pub fn demo_1() {
  let s1 = String::from("hello, world!");
  let len = calculate_length(&s1);

  println!("{}'s length is {}", s1, len);
}

fn change(some_string: &mut String) {
  some_string.push_str(", world!")
}

pub fn demo_2() {
  let mut s = String::from("hello");

  change(&mut s);
}
